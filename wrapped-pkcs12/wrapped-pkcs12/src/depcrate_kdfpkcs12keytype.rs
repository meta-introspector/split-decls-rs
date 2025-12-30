// Generated macro for Pkcs12KeyType (enum)
macro_rules! Depcrate_kdfPkcs12KeyType {
() => {
// Module: crate::kdf
// Provides: {"Pkcs12KeyType"}
// Dependencies: {}
# [doc = " Specify the usage type of the generated key"] # [doc = " This allows to derive distinct encryption keys, IVs and MAC from the same password or text"] # [doc = " string."] pub enum Pkcs12KeyType { # [doc = " Use key for encryption"] EncryptionKey = 1 , # [doc = " Use key as initial vector"] Iv = 2 , # [doc = " Use key as MAC"] Mac = 3 , }
};
}
