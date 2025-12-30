// Generated macro for impl_160 (impl)
macro_rules! Depcrate_arcimpl_160 {
() => {
// Module: crate::arc
// Provides: {"impl_160"}
// Dependencies: {}
impl < T : ? Sized + Ord > Ord for Arc < T > { fn cmp (& self , other : & Arc < T >) -> Ordering { (* * self) . cmp (& * * other) } }
};
}
