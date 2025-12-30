// Generated macro for ControlFlow (trait)
macro_rules! Depcrate_controlControlFlow {
() => {
// Module: crate::control
// Provides: {"ControlFlow"}
// Dependencies: {}
# [doc = " Control flow for callbacks."] # [doc = ""] # [doc = " The empty return value `()` is equivalent to continue."] # [allow (clippy :: module_name_repetitions)] pub trait ControlFlow { fn continuing () -> Self ; # [inline] fn should_break (& self) -> bool { false } }
};
}
