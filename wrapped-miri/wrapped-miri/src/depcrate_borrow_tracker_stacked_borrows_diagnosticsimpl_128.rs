// Generated macro for impl_128 (impl)
macro_rules! Depcrate_borrow_tracker_stacked_borrows_diagnosticsimpl_128 {
() => {
// Module: crate::borrow_tracker::stacked_borrows::diagnostics
// Provides: {"impl_128"}
// Dependencies: {}
impl RetagInfo { fn summary (& self) -> String { let mut s = match self . cause { RetagCause :: Normal => "retag" , RetagCause :: FnEntry => "function-entry retag" , RetagCause :: InPlaceFnPassing => "in-place function argument/return passing protection" , RetagCause :: TwoPhase => "two-phase retag" , } . to_string () ; if self . in_field { s . push_str (" (of a reference/box inside this compound value)") ; } s } }
};
}
