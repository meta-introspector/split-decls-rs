// Generated macro for impl_1268 (impl)
macro_rules! Depcrate_stackimpl_1268 {
() => {
// Module: crate::stack
// Provides: {"impl_1268"}
// Dependencies: {}
impl < T : Stackable > Deref for Stack < T > { type Target = StackRef < T > ; fn deref (& self) -> & StackRef < T > { unsafe { StackRef :: from_ptr (self . 0) } } }
};
}
