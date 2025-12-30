// Generated macro for impl_354 (impl)
macro_rules! Depcrate_options_themeimpl_354 {
() => {
// Module: crate::options::theme
// Provides: {"impl_354"}
// Dependencies: {}
impl Definitions { fn deduce < V : Vars > (vars : & V) -> Self { let ls = vars . get (vars :: LS_COLORS) . map (| e | e . to_string_lossy () . to_string ()) ; let exa = vars . get_with_fallback (vars :: EZA_COLORS , vars :: EXA_COLORS) . map (| e | e . to_string_lossy () . to_string ()) ; Self { ls , exa } } }
};
}
