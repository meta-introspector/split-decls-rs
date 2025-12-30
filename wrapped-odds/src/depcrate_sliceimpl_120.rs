// Generated macro for impl_120 (impl)
macro_rules! Depcrate_sliceimpl_120 {
() => {
// Module: crate::slice
// Provides: {"impl_120"}
// Dependencies: {}
impl < T > SliceFind for [T] { type Item = T ; fn find < U : ? Sized > (& self , elt : & U) -> Option < usize > where Self :: Item : PartialEq < U > , { SliceIter :: from (self) . position (move | x | * x == * elt) } fn rfind < U : ? Sized > (& self , elt : & U) -> Option < usize > where Self :: Item : PartialEq < U > , { SliceIter :: from (self) . rposition (move | x | * x == * elt) } }
};
}
