// Generated macro for extract_by_ref (function)
macro_rules! Depcrate_parse_by_refextract_by_ref {
() => {
// Module: crate::parse::by_ref
// Provides: {"extract_by_ref"}
// Dependencies: {}
pub (crate) fn extract_by_ref (item_fn : & mut ItemFn) -> Result < Vec < Pat > , ErrorsVec > { let mut extractor = JustOnceFnArgAttributeExtractor :: from ("by_ref") ; extractor . visit_item_fn_mut (item_fn) ; extractor . take () }
};
}
