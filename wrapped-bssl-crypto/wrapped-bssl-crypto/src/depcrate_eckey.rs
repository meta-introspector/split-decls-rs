// Generated macro for Key (struct)
macro_rules! Depcrate_ecKey {
() => {
// Module: crate::ec
// Provides: {"Key"}
// Dependencies: {}
# [doc = " Key holds both a public and private key. While BoringSSL allows an `EC_KEY`"] # [doc = " to also be a) empty, b) holding only a private scalar, or c) holding only"] pub (crate) struct Key (* mut bssl_sys :: EC_KEY) ;
};
}
