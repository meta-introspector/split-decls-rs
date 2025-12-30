// Generated macro for impl_299 (impl)
macro_rules! Depcrate_matchers_container_eq_matcherimpl_299 {
() => {
// Module: crate::matchers::container_eq_matcher
// Provides: {"impl_299"}
// Dependencies: {}
impl < ExpectedContainerT > ContainerEqMatcher < ExpectedContainerT > { # [doc = " Match container equality, but ignoring element order."] pub fn ignore_order (self) -> IgnoringOrder < ExpectedContainerT > { IgnoringOrder { expected : self . expected } } }
};
}
