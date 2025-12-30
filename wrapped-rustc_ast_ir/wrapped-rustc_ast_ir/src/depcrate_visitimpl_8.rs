// Generated macro for impl_8 (impl)
macro_rules! Depcrate_visitimpl_8 {
() => {
// Module: crate::visit
// Provides: {"impl_8"}
// Dependencies: {}
impl < T > VisitorResult for ControlFlow < T > { type Residual = T ; fn output () -> Self { ControlFlow :: Continue (()) } fn from_residual (residual : Self :: Residual) -> Self { ControlFlow :: Break (residual) } fn from_branch (b : Self) -> Self { b } fn branch (self) -> Self { self } }
};
}
