// Generated macro for impl_93 (impl)
macro_rules! Depcrate_arrayvecimpl_93 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_93"}
// Dependencies: {}
impl < T , const CAP : usize > Clone for IntoIter < T , CAP > where T : Clone , { fn clone (& self) -> IntoIter < T , CAP > { let mut v = ArrayVec :: new () ; v . extend_from_slice (& self . v [self . index ..]) ; v . into_iter () } }
};
}
