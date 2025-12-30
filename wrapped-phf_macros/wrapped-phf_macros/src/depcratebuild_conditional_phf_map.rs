// Generated macro for build_conditional_phf_map (function)
macro_rules! Depcratebuild_conditional_phf_map {
() => {
// Module: crate
// Provides: {"build_conditional_phf_map"}
// Dependencies: {}
fn build_conditional_phf_map (entries : & [Entry]) -> proc_macro2 :: TokenStream { build_conditional_phf (entries , build_map , quote ! { phf :: Map { key : 0 , disps : & [] , entries : & [] , } } ,) }
};
}
