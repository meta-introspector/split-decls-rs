// Generated macro for VisitorResult (trait)
macro_rules! Depcrate_visitVisitorResult {
() => {
// Module: crate::visit
// Provides: {"VisitorResult"}
// Dependencies: {}
# [doc = " Similar to the `Try` trait, but also implemented for `()`."] pub trait VisitorResult { type Residual ; fn output () -> Self ; fn from_residual (residual : Self :: Residual) -> Self ; fn from_branch (b : ControlFlow < Self :: Residual >) -> Self ; fn branch (self) -> ControlFlow < Self :: Residual > ; }
};
}
