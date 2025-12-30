// Generated macro for pretty_error (function)
macro_rules! Depcrate_print_errorspretty_error {
() => {
// Module: crate::print_errors
// Provides: {"pretty_error"}
// Dependencies: {}
# [doc = " Pretty-print a Cranelift error."] pub fn pretty_error (func : & ir :: Function , err : CodegenError) -> String { if let CodegenError :: Verifier (e) = err { pretty_verifier_error (func , None , e) } else { err . to_string () } }
};
}
