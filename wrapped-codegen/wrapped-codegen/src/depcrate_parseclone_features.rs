// Generated macro for clone_features (function)
macro_rules! Depcrate_parseclone_features {
() => {
// Module: crate::parse
// Provides: {"clone_features"}
// Dependencies: {}
fn clone_features (features : & [Attribute]) -> Vec < Attribute > { features . iter () . map (| attr | parse_quote ! (# attr)) . collect () }
};
}
