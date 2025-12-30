// Generated macro for impl_619 (impl)
macro_rules! Depcrate_sso_setimpl_619 {
() => {
// Module: crate::sso::set
// Provides: {"impl_619"}
// Dependencies: {}
impl < T > fmt :: Debug for SsoHashSet < T > where T : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_set () . entries (self . iter ()) . finish () } }
};
}
