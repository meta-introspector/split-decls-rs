// Generated macro for build_conditional_phf_ordered_set (function)
macro_rules! Depcratebuild_conditional_phf_ordered_set {
() => {
// Module: crate
// Provides: {"build_conditional_phf_ordered_set"}
// Dependencies: {}
fn build_conditional_phf_ordered_set (entries : & [Entry]) -> proc_macro2 :: TokenStream { let map_tokens = build_conditional_phf_ordered_map (entries) ; quote ! (phf :: OrderedSet { map : # map_tokens }) }
};
}
