// Generated macro for impl_98 (impl)
macro_rules! Depcrate_agreementimpl_98 {
() => {
// Module: crate::agreement
// Provides: {"impl_98"}
// Dependencies: {}
impl Debug for PrivateKey { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , fmt :: Error > { f . write_str (& format ! ("PrivateKey {{ algorithm: {:?} }}" , self . inner_key . algorithm ())) } }
};
}
