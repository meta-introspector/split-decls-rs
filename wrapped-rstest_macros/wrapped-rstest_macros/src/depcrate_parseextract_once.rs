// Generated macro for extract_once (function)
macro_rules! Depcrate_parseextract_once {
() => {
// Module: crate::parse
// Provides: {"extract_once"}
// Dependencies: {}
pub (crate) fn extract_once (item_fn : & mut ItemFn) -> Result < Option < syn :: Attribute > , ErrorsVec > { let mut extractor = JustOnceFnAttributeExtractor :: from ("once") ; extractor . visit_item_fn_mut (item_fn) ; extractor . take () }
};
}
