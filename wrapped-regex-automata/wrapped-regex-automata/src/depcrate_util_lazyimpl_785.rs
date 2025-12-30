// Generated macro for impl_785 (impl)
macro_rules! Depcrate_util_lazyimpl_785 {
() => {
// Module: crate::util::lazy
// Provides: {"impl_785"}
// Dependencies: {}
impl < T , F : Fn () -> T > core :: ops :: Deref for Lazy < T , F > { type Target = T ; fn deref (& self) -> & T { Lazy :: get (self) } }
};
}
