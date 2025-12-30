// Generated macro for extract_context (function)
macro_rules! Depcrate_parse_contextextract_context {
() => {
// Module: crate::parse::context
// Provides: {"extract_context"}
// Dependencies: {}
pub (crate) fn extract_context (item_fn : & mut ItemFn) -> Result < Vec < Pat > , ErrorsVec > { let mut extractor = JustOnceFnArgAttributeExtractor :: from ("context") ; extractor . visit_item_fn_mut (item_fn) ; extractor . take () }
};
}
