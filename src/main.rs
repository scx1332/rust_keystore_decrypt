use crate::keystore::decrypt_key;

mod keystore;


fn main() {
    let keystore_no_password = r#"
{
  "id": "777694ee-120f-4b96-b4ff-752f46552394",
  "version": 3,
  "crypto": {
    "cipher": "aes-128-ctr",
    "cipherparams": {
      "iv": "4efee3bb098c87c56052c20036848b5a"
    },
    "ciphertext": "7ba29006b35115d9d98d08aa20804332d3147ecbb66fb5813fdc5c6ad553c605",
    "kdf": "pbkdf2",
    "kdfparams": {
      "c": 10240,
      "dklen": 32,
      "prf": "hmac-sha256",
      "salt": "b538883c64ffa4dbcea014bb42a3f498df57e54951166c078cba1ae4c63db623"
    },
    "mac": "7df44c285f0af16c58b22f434e391d51034bf0367fd61bf330d8ab02b59641be"
  },
  "address": "5afa3ec4d616b059e9f0abd77e0a336e80dfb77e"
}
    "#;
    let secret = decrypt_key(keystore_no_password, "k1h$&qT@&seZy5VS").unwrap();
    let secret = hex::encode(secret);
    println!("Private key: {}", secret);

}
