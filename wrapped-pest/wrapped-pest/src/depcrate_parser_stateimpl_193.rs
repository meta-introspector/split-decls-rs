// Generated macro for impl_193 (impl)
macro_rules! Depcrate_parser_stateimpl_193 {
() => {
// Module: crate::parser_state
// Provides: {"impl_193"}
// Dependencies: {}
impl Default for CallLimitTracker { fn default () -> Self { let limit = CALL_LIMIT . load (Ordering :: Relaxed) ; let current_call_limit = if limit > 0 { Some ((0 , limit)) } else { None } ; Self { current_call_limit } } }
};
}
