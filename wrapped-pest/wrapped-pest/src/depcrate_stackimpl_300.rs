// Generated macro for impl_300 (impl)
macro_rules! Depcrate_stackimpl_300 {
() => {
// Module: crate::stack
// Provides: {"impl_300"}
// Dependencies: {}
impl < T : Clone > Index < Range < usize > > for Stack < T > { type Output = [T] ; fn index (& self , range : Range < usize >) -> & [T] { self . cache . index (range) } }
};
}
