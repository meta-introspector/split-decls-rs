// Generated macro for impl_96 (impl)
macro_rules! Depcrate_stackimpl_96 {
() => {
// Module: crate::stack
// Provides: {"impl_96"}
// Dependencies: {}
impl < T > std :: ops :: Deref for StackBox < T > { type Target = T ; fn deref (& self) -> & T { unsafe { self . ptr . as_ref () } } }
};
}
