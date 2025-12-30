// Generated macro for impl_97 (impl)
macro_rules! Depcrate_stackimpl_97 {
() => {
// Module: crate::stack
// Provides: {"impl_97"}
// Dependencies: {}
impl < T > std :: ops :: DerefMut for StackBox < T > { fn deref_mut (& mut self) -> & mut T { unsafe { & mut * self . ptr . as_mut () } } }
};
}
