// Generated macro for impl_15 (impl)
macro_rules! Depcrate_controlimpl_15 {
() => {
// Module: crate::control
// Provides: {"impl_15"}
// Dependencies: {}
impl < E > ControlFlow for Result < () , E > { fn continuing () -> Self { Ok (()) } fn should_break (& self) -> bool { self . is_err () } }
};
}
