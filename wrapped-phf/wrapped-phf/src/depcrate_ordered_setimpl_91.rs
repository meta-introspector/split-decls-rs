// Generated macro for impl_91 (impl)
macro_rules! Depcrate_ordered_setimpl_91 {
() => {
// Module: crate::ordered_set
// Provides: {"impl_91"}
// Dependencies: {}
impl < T > fmt :: Debug for OrderedSet < T > where T : fmt :: Debug , { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt . debug_set () . entries (self) . finish () } }
};
}
