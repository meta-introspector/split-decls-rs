// Generated macro for pretty_verifier_error (function)
macro_rules! Depcrate_print_errorspretty_verifier_error {
() => {
// Module: crate::print_errors
// Provides: {"pretty_verifier_error"}
// Dependencies: {}
# [doc = " Pretty-print a verifier error."] pub fn pretty_verifier_error < 'a > (func : & ir :: Function , func_w : Option < Box < dyn FuncWriter + 'a > > , errors : VerifierErrors ,) -> String { let mut errors = errors . 0 ; let mut w = String :: new () ; let num_errors = errors . len () ; decorate_function (& mut PrettyVerifierError (func_w . unwrap_or_else (| | Box :: new (PlainWriter)) , & mut errors) , & mut w , func ,) . unwrap () ; writeln ! (w , "\n; {} verifier error{} detected (see above). Compilation aborted." , num_errors , if num_errors == 1 { "" } else { "s" }) . unwrap () ; w }
};
}
