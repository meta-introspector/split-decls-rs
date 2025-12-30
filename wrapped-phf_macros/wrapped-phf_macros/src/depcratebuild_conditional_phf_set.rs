// Generated macro for build_conditional_phf_set (function)
macro_rules! Depcratebuild_conditional_phf_set {
() => {
// Module: crate
// Provides: {"build_conditional_phf_set"}
// Dependencies: {}
fn build_conditional_phf_set (entries : & [Entry]) -> proc_macro2 :: TokenStream { let map_tokens = build_conditional_phf_map (entries) ; quote ! (phf :: Set { map : # map_tokens }) }
};
}
