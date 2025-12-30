// Generated macro for impl_162 (impl)
macro_rules! Depcrate_posimpl_162 {
() => {
// Module: crate::pos
// Provides: {"impl_162"}
// Dependencies: {}
impl < T : Ord > Ord for Positioned < T > { fn cmp (& self , other : & Self) -> Ordering { self . node . cmp (& other . node) } }
};
}
