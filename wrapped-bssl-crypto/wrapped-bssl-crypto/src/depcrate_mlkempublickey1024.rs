// Generated macro for PublicKey1024 (struct)
macro_rules! Depcrate_mlkemPublicKey1024 {
() => {
// Module: crate::mlkem
// Provides: {"PublicKey1024"}
// Dependencies: {}
# [doc = " An ML-KEM-1024 public key."] # [doc = ""] # [doc = " Use ML-KEM-768 unless you have a good reason to need this larger size."] pub struct PublicKey1024 (Box < bssl_sys :: MLKEM1024_public_key >) ;
};
}
