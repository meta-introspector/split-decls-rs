// Generated macro for debug_print_vec_of_tokenizable (function)
macro_rules! Depcrate_internals_test_helpersdebug_print_vec_of_tokenizable {
() => {
// Module: crate::internals::test_helpers
// Provides: {"debug_print_vec_of_tokenizable"}
// Dependencies: {}
pub fn debug_print_vec_of_tokenizable < T : ToTokens > (optional : Option < Vec < T > >) -> String { let mut s = String :: new () ; if let Some (vec) = optional { for element in vec { writeln ! (& mut s , "{}" , element . to_token_stream ()) . unwrap () ; } } else { write ! (& mut s , "None") . unwrap () ; } s }
};
}
