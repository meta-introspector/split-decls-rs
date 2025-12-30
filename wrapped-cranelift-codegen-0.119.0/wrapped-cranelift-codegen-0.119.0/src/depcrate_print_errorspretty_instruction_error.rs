// Generated macro for pretty_instruction_error (function)
macro_rules! Depcrate_print_errorspretty_instruction_error {
() => {
// Module: crate::print_errors
// Provides: {"pretty_instruction_error"}
// Dependencies: {}
# [doc = " Pretty-print a function verifier error for a given instruction."] fn pretty_instruction_error (w : & mut dyn Write , func : & Function , aliases : & SecondaryMap < Value , Vec < Value > > , cur_inst : Inst , indent : usize , func_w : & mut dyn FuncWriter , errors : & mut Vec < VerifierError > ,) -> fmt :: Result { let mut s = String :: new () ; func_w . write_instruction (& mut s , func , aliases , cur_inst , indent) ? ; write ! (w , "{s}") ? ; let mut i = 0 ; let mut printed_error = false ; while i != errors . len () { match errors [i] . location { ir :: entities :: AnyEntity :: Inst (inst) if inst == cur_inst => { if ! printed_error { print_arrow (w , & s) ? ; printed_error = true ; } let err = errors . remove (i) ; print_error (w , err) ? ; } _ => i += 1 , } } if printed_error { w . write_char ('\n') ? ; } Ok (()) }
};
}
