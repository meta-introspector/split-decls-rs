// Generated macro for print_error (function)
macro_rules! Depcrate_print_errorsprint_error {
() => {
// Module: crate::print_errors
// Provides: {"print_error"}
// Dependencies: {}
# [doc = " Prints:"] # [doc = "    ; error: [ERROR BODY]"] fn print_error (w : & mut dyn Write , err : VerifierError) -> fmt :: Result { writeln ! (w , "; error: {}" , err . to_string ()) ? ; Ok (()) }
};
}
