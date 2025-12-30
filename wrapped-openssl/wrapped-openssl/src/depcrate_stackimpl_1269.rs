// Generated macro for impl_1269 (impl)
macro_rules! Depcrate_stackimpl_1269 {
() => {
// Module: crate::stack
// Provides: {"impl_1269"}
// Dependencies: {}
impl < T : Stackable > DerefMut for Stack < T > { fn deref_mut (& mut self) -> & mut StackRef < T > { unsafe { StackRef :: from_ptr_mut (self . 0) } } }
};
}
