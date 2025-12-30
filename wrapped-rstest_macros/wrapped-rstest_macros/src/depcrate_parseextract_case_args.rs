// Generated macro for extract_case_args (function)
macro_rules! Depcrate_parseextract_case_args {
() => {
// Module: crate::parse
// Provides: {"extract_case_args"}
// Dependencies: {}
pub (crate) fn extract_case_args (item_fn : & mut ItemFn) -> Result < Vec < Pat > , ErrorsVec > { let mut extractor = JustOnceFnArgAttributeExtractor :: from ("case") ; extractor . visit_item_fn_mut (item_fn) ; extractor . take () }
};
}
