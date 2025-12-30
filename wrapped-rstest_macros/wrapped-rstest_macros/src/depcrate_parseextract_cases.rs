// Generated macro for extract_cases (function)
macro_rules! Depcrate_parseextract_cases {
() => {
// Module: crate::parse
// Provides: {"extract_cases"}
// Dependencies: {}
pub (crate) fn extract_cases (item_fn : & mut ItemFn) -> Result < Vec < TestCase > , ErrorsVec > { let mut cases_extractor = CasesFunctionExtractor :: default () ; cases_extractor . visit_item_fn_mut (item_fn) ; if cases_extractor . 1 . is_empty () { Ok (cases_extractor . 0) } else { Err (cases_extractor . 1 . into ()) } }
};
}
