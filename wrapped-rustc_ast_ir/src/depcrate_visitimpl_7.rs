// Generated macro for impl_7 (impl)
macro_rules! Depcrate_visitimpl_7 {
() => {
// Module: crate::visit
// Provides: {"impl_7"}
// Dependencies: {}
impl VisitorResult for () { # [cfg (feature = "nightly")] type Residual = ! ; # [cfg (not (feature = "nightly"))] type Residual = core :: convert :: Infallible ; fn output () -> Self { } fn from_residual (_ : Self :: Residual) -> Self { } fn from_branch (_ : ControlFlow < Self :: Residual >) -> Self { } fn branch (self) -> ControlFlow < Self :: Residual > { ControlFlow :: Continue (()) } }
};
}
