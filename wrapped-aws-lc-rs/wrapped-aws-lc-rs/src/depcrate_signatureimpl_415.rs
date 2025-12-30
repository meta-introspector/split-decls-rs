// Generated macro for impl_415 (impl)
macro_rules! Depcrate_signatureimpl_415 {
() => {
// Module: crate::signature
// Provides: {"impl_415"}
// Dependencies: {}
impl Debug for ParsedPublicKey { fn fmt (& self , f : & mut Formatter < '_ >) -> core :: fmt :: Result { f . write_str (& format ! ("ParsedPublicKey {{ algorithm: {:?}, bytes: \"{}\" }}" , self . algorithm , hex :: encode (self . bytes . as_ref ()))) } }
};
}
