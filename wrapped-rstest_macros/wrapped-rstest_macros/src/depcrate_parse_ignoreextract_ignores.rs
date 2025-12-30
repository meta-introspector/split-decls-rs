// Generated macro for extract_ignores (function)
macro_rules! Depcrate_parse_ignoreextract_ignores {
() => {
// Module: crate::parse::ignore
// Provides: {"extract_ignores"}
// Dependencies: {}
pub (crate) fn extract_ignores (item_fn : & mut ItemFn) -> Result < Vec < Pat > , ErrorsVec > { let mut extractor = JustOnceFnArgAttributeExtractor :: from ("ignore") ; extractor . visit_item_fn_mut (item_fn) ; extractor . take () }
};
}
