// Generated macro for complete_custom_arg_value (function)
macro_rules! Depcrate_engine_completecomplete_custom_arg_value {
() => {
// Module: crate::engine::complete
// Provides: {"complete_custom_arg_value"}
// Dependencies: {}
fn complete_custom_arg_value (value : & OsStr , completer : & ArgValueCandidates ,) -> Vec < CompletionCandidate > { debug ! ("complete_custom_arg_value: completer={completer:?}, value={value:?}") ; let mut values = completer . candidates () ; values . retain (| comp | comp . get_value () . starts_with (& value . to_string_lossy ())) ; values }
};
}
