// Generated macro for impl_161 (impl)
macro_rules! Depcrate_posimpl_161 {
() => {
// Module: crate::pos
// Provides: {"impl_161"}
// Dependencies: {}
impl < T : PartialOrd > PartialOrd for Positioned < T > { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { self . node . partial_cmp (& other . node) } }
};
}
