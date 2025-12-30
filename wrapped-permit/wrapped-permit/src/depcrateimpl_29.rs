// Generated macro for impl_29 (impl)
macro_rules! Depcrateimpl_29 {
() => {
// Module: crate
// Provides: {"impl_29"}
// Dependencies: {}
impl Debug for Permit { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , core :: fmt :: Error > { write ! (f , "Permit{{revoked={},num_subs={}}}" , self . is_revoked () , Arc :: weak_count (& self . node)) } }
};
}
