// Generated macro for complete_external_subcommand (function)
macro_rules! Depcrate_engine_completecomplete_external_subcommand {
() => {
// Module: crate::engine::complete
// Provides: {"complete_external_subcommand"}
// Dependencies: {}
fn complete_external_subcommand (value : & str , completer : & SubcommandCandidates ,) -> Vec < CompletionCandidate > { debug ! ("complete_custom_arg_value: completer={completer:?}, value={value:?}") ; let mut values = Vec :: new () ; let custom_arg_values = completer . candidates () ; values . extend (custom_arg_values) ; values . retain (| comp | comp . get_value () . starts_with (value)) ; values }
};
}
