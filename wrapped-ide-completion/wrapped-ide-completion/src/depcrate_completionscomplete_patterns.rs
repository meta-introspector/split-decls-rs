// Generated macro for complete_patterns (function)
macro_rules! Depcrate_completionscomplete_patterns {
() => {
// Module: crate::completions
// Provides: {"complete_patterns"}
// Dependencies: {}
fn complete_patterns (acc : & mut Completions , ctx : & CompletionContext < '_ > , pattern_ctx : & PatternContext ,) { flyimport :: import_on_the_fly_pat (acc , ctx , pattern_ctx) ; fn_param :: complete_fn_param (acc , ctx , pattern_ctx) ; pattern :: complete_pattern (acc , ctx , pattern_ctx) ; record :: complete_record_pattern_fields (acc , ctx , pattern_ctx) ; }
};
}
