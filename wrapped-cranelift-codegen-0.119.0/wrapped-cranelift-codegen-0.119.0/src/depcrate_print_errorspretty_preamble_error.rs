// Generated macro for pretty_preamble_error (function)
macro_rules! Depcrate_print_errorspretty_preamble_error {
() => {
// Module: crate::print_errors
// Provides: {"pretty_preamble_error"}
// Dependencies: {}
fn pretty_preamble_error (w : & mut dyn Write , func : & Function , entity : AnyEntity , value : & dyn fmt :: Display , maybe_fact : Option < & Fact > , func_w : & mut dyn FuncWriter , errors : & mut Vec < VerifierError > ,) -> fmt :: Result { let mut s = String :: new () ; func_w . write_entity_definition (& mut s , func , entity , value , maybe_fact) ? ; write ! (w , "{s}") ? ; let mut i = 0 ; let mut printed_error = false ; while i != errors . len () { if entity == errors [i] . location { if ! printed_error { print_arrow (w , & s) ? ; printed_error = true ; } let err = errors . remove (i) ; print_error (w , err) ? ; } else { i += 1 } } if printed_error { w . write_char ('\n') ? ; } Ok (()) }
};
}
