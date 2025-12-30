// Generated macro for impl_1132 (impl)
macro_rules! Depcrate_signatureimpl_1132 {
() => {
// Module: crate::signature
// Provides: {"impl_1132"}
// Dependencies: {}
impl < 'a > fmt :: Display for Signature < 'a > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{} <{}>" , String :: from_utf8_lossy (self . name_bytes ()) , String :: from_utf8_lossy (self . email_bytes ())) } }
};
}
