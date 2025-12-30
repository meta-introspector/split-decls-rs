// Generated macro for impl_697 (impl)
macro_rules! Depcrate_validation_contextimpl_697 {
() => {
// Module: crate::validation::context
// Provides: {"impl_697"}
// Dependencies: {}
impl RuleError { # [doc (hidden)] pub fn new (message : & str , locations : & [SourcePosition]) -> Self { Self { message : message . into () , locations : locations . to_vec () , } } # [doc = " Access the message for a validation error"] pub fn message (& self) -> & str { & self . message } # [doc = " Access the positions of the validation error"] # [doc = ""] # [doc = " All validation errors contain at least one source position, but some"] # [doc = " validators supply extra context through multiple positions."] pub fn locations (& self) -> & [SourcePosition] { & self . locations } }
};
}
