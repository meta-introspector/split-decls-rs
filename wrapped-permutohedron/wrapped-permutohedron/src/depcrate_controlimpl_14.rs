// Generated macro for impl_14 (impl)
macro_rules! Depcrate_controlimpl_14 {
() => {
// Module: crate::control
// Provides: {"impl_14"}
// Dependencies: {}
impl < B > ControlFlow for Control < B > { fn continuing () -> Self { Self :: Continue } fn should_break (& self) -> bool { matches ! (self , Self :: Break (_)) } }
};
}
