// Generated macro for extract_global_awt (function)
macro_rules! Depcrate_parse_futureextract_global_awt {
() => {
// Module: crate::parse::future
// Provides: {"extract_global_awt"}
// Dependencies: {}
pub (crate) fn extract_global_awt (item_fn : & mut ItemFn) -> Result < bool , ErrorsVec > { let mut extractor = JustOnceFnAttributeExtractor :: < GlobalAwtBuilder > :: new ("awt") ; extractor . visit_item_fn_mut (item_fn) ; extractor . take () . map (| inner | inner . is_some ()) }
};
}
