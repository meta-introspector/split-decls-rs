// Generated macro for impl_47 (impl)
macro_rules! Depcrate_visit_dfsvisitimpl_47 {
() => {
// Module: crate::visit::dfsvisit
// Provides: {"impl_47"}
// Dependencies: {}
impl < C : ControlFlow , E > ControlFlow for Result < C , E > { fn continuing () -> Self { Ok (C :: continuing ()) } fn should_break (& self) -> bool { if let Ok (ref c) = * self { c . should_break () } else { true } } fn should_prune (& self) -> bool { if let Ok (ref c) = * self { c . should_prune () } else { false } } }
};
}
