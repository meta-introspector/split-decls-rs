use rustc_span::Span;
use rustc_expand_base_lib_custom_macros::CustomDiagnostic;
use rustc_errors::codes::*; // original
//use rustc_error_codes::E0000; // Import the error code

#[derive(CustomDiagnostic)]
//#[diag(expand_base_lib_errors_trace_macro_base, code = E0000)] // Placeholder error code
pub struct TraceMacroBase {
//    #[primary_span]
    pub span: Span,
}
