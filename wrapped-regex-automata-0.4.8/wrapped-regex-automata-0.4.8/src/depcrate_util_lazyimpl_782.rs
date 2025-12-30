// Generated macro for impl_782 (impl)
macro_rules! Depcrate_util_lazyimpl_782 {
() => {
// Module: crate::util::lazy
// Provides: {"impl_782"}
// Dependencies: {}
impl < T , F : Fn () -> T > core :: ops :: Deref for Lazy < T , F > { type Target = T ; fn deref (& self) -> & T { Lazy :: get (self) } }
};
}
