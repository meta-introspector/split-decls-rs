// Generated macro for impl_163 (impl)
macro_rules! Depcrateimpl_163 {
() => {
// Module: crate
// Provides: {"impl_163"}
// Dependencies: {}
impl < T : Clone , const N : usize > Clone for IntoIter < T , N > { # [inline] fn clone (& self) -> IntoIter < T , N > { SmallVec :: from (self . as_slice ()) . into_iter () } }
};
}
