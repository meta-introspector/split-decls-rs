// Generated macro for build_oibits (function)
macro_rules! Depcrate_buildbuild_oibits {
() => {
// Module: crate::build
// Provides: {"build_oibits"}
// Dependencies: {}
fn build_oibits (crate_ : & Path , input : & ForeignType) -> TokenStream { let oibits = input . oibits . iter () . map (| t | build_oibit (crate_ , input , t)) ; quote ! { # (# oibits) * } }
};
}
