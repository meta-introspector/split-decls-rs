// Generated macro for impl_418 (impl)
macro_rules! Depcrate_signatureimpl_418 {
() => {
// Module: crate::signature
// Provides: {"impl_418"}
// Dependencies: {}
impl < B : AsRef < [u8] > > Debug for UnparsedPublicKey < B > { fn fmt (& self , f : & mut Formatter < '_ >) -> core :: fmt :: Result { f . write_str (& format ! ("UnparsedPublicKey {{ algorithm: {:?}, bytes: \"{}\" }}" , self . algorithm , hex :: encode (self . bytes . as_ref ()))) } }
};
}
