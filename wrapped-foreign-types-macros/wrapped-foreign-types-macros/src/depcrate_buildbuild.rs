// Generated macro for build (function)
macro_rules! Depcrate_buildbuild {
() => {
// Module: crate::build
// Provides: {"build"}
// Dependencies: {}
pub fn build (input : Input) -> TokenStream { let types = input . types . iter () . map (| t | build_foreign_type (& input . crate_ , t)) ; quote ! { # (# types) * } }
};
}
