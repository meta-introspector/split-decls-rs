// Generated macro for build_conditional_phf_ordered_map (function)
macro_rules! Depcratebuild_conditional_phf_ordered_map {
() => {
// Module: crate
// Provides: {"build_conditional_phf_ordered_map"}
// Dependencies: {}
fn build_conditional_phf_ordered_map (entries : & [Entry]) -> proc_macro2 :: TokenStream { build_conditional_phf (entries , build_ordered_map , quote ! { phf :: OrderedMap { key : 0 , disps : & [] , idxs : & [] , entries : & [] , } } ,) }
};
}
