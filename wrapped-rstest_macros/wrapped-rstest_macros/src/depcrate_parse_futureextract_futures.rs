// Generated macro for extract_futures (function)
macro_rules! Depcrate_parse_futureextract_futures {
() => {
// Module: crate::parse::future
// Provides: {"extract_futures"}
// Dependencies: {}
pub (crate) fn extract_futures (item_fn : & mut ItemFn) -> Result < Vec < (Pat , FutureArg) > , ErrorsVec > { let mut extractor = JustOnceFnArgAttributeExtractor :: < FutureBuilder > :: new ("future") ; extractor . visit_item_fn_mut (item_fn) ; extractor . take () }
};
}
