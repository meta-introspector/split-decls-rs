// Generated macro for ControlFlow (trait)
macro_rules! Depcrate_visit_dfsvisitControlFlow {
() => {
// Module: crate::visit::dfsvisit
// Provides: {"ControlFlow"}
// Dependencies: {}
# [doc = " Control flow for callbacks."] # [doc = ""] # [doc = " The empty return value `()` is equivalent to continue."] pub trait ControlFlow { fn continuing () -> Self ; fn should_break (& self) -> bool ; fn should_prune (& self) -> bool ; }
};
}
