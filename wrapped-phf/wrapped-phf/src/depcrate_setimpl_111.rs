// Generated macro for impl_111 (impl)
macro_rules! Depcrate_setimpl_111 {
() => {
// Module: crate::set
// Provides: {"impl_111"}
// Dependencies: {}
impl < T > fmt :: Debug for Set < T > where T : fmt :: Debug , { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt . debug_set () . entries (self) . finish () } }
};
}
