// Generated macro for impl_118 (impl)
macro_rules! Depcrate_agreementimpl_118 {
() => {
// Module: crate::agreement
// Provides: {"impl_118"}
// Dependencies: {}
impl < B : Debug + AsRef < [u8] > > Debug for UnparsedPublicKey < B > { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , fmt :: Error > { f . write_str (& format ! ("UnparsedPublicKey {{ algorithm: {:?}, bytes: {:?} }}" , self . alg , hex :: encode (self . bytes . as_ref ()))) } }
};
}
