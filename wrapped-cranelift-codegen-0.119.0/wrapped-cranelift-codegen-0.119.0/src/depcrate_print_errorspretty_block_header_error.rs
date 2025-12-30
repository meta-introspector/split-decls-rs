// Generated macro for pretty_block_header_error (function)
macro_rules! Depcrate_print_errorspretty_block_header_error {
() => {
// Module: crate::print_errors
// Provides: {"pretty_block_header_error"}
// Dependencies: {}
# [doc = " Pretty-print a function verifier error for a given block."] fn pretty_block_header_error (w : & mut dyn Write , func : & Function , cur_block : Block , indent : usize , func_w : & mut dyn FuncWriter , errors : & mut Vec < VerifierError > ,) -> fmt :: Result { let mut s = String :: new () ; func_w . write_block_header (& mut s , func , cur_block , indent) ? ; write ! (w , "{s}") ? ; let mut i = 0 ; let mut printed_error = false ; while i != errors . len () { match errors [i] . location { ir :: entities :: AnyEntity :: Block (block) if block == cur_block => { if ! printed_error { print_arrow (w , & s) ? ; printed_error = true ; } let err = errors . remove (i) ; print_error (w , err) ? ; } _ => i += 1 , } } if printed_error { w . write_char ('\n') ? ; } Ok (()) }
};
}
