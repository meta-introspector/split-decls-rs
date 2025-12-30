// Generated macro for impl_46 (impl)
macro_rules! Depcrate_visit_dfsvisitimpl_46 {
() => {
// Module: crate::visit::dfsvisit
// Provides: {"impl_46"}
// Dependencies: {}
impl < B > ControlFlow for Control < B > { fn continuing () -> Self { Control :: Continue } fn should_break (& self) -> bool { matches ! (* self , Control :: Break (_)) } fn should_prune (& self) -> bool { matches ! (* self , Control :: Prune) } }
};
}
