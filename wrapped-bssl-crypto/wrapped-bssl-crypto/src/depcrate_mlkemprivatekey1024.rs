// Generated macro for PrivateKey1024 (struct)
macro_rules! Depcrate_mlkemPrivateKey1024 {
() => {
// Module: crate::mlkem
// Provides: {"PrivateKey1024"}
// Dependencies: {}
# [doc = " An ML-KEM-1024 private key."] # [doc = ""] # [doc = " Use ML-KEM-768 unless you have a good reason to need this larger size."] pub struct PrivateKey1024 (Box < bssl_sys :: MLKEM1024_private_key >) ;
};
}
