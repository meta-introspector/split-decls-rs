// Generated macro for pretty_print_syn_str (function)
macro_rules! Depcrate_internals_test_helperspretty_print_syn_str {
() => {
// Module: crate::internals::test_helpers
// Provides: {"pretty_print_syn_str"}
// Dependencies: {}
pub fn pretty_print_syn_str (input : & TokenStream) -> syn :: Result < String > { let input = format ! ("{}" , quote ! (# input)) ; let syn_file = syn :: parse_str :: < syn :: File > (& input) ? ; Ok (prettyplease :: unparse (& syn_file)) }
};
}
