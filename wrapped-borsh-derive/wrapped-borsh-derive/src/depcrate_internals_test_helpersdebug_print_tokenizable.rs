// Generated macro for debug_print_tokenizable (function)
macro_rules! Depcrate_internals_test_helpersdebug_print_tokenizable {
() => {
// Module: crate::internals::test_helpers
// Provides: {"debug_print_tokenizable"}
// Dependencies: {}
pub fn debug_print_tokenizable < T : ToTokens > (optional : Option < T >) -> String { let mut s = String :: new () ; if let Some (type_) = optional { writeln ! (& mut s , "{}" , type_ . to_token_stream ()) . unwrap () ; } else { write ! (& mut s , "None") . unwrap () ; } s }
};
}
