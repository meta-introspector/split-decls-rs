// Generated macro for impl_194 (impl)
macro_rules! Depcrate_parser_stateimpl_194 {
() => {
// Module: crate::parser_state
// Provides: {"impl_194"}
// Dependencies: {}
impl CallLimitTracker { fn limit_reached (& self) -> bool { self . current_call_limit . is_some_and (| (current , limit) | current >= limit) } fn increment_depth (& mut self) { if let Some ((current , _)) = & mut self . current_call_limit { * current += 1 ; } } }
};
}
