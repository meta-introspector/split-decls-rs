use rustc_span::Span;
use rustc_expand_base_lib_custom_macros::CustomSubdiagnostic;
use rustc_errors::codes::*; // Added for consistency, even if not strictly needed by Subdiagnostic

#[derive(CustomSubdiagnostic)]
//#[note(expand_base_lib_trace_macro_note)]
pub struct TraceMacroNote {
//    #[primary_span]
    pub span: Span,
    pub message: String,
}
