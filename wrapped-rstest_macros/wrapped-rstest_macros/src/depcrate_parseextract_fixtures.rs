// Generated macro for extract_fixtures (function)
macro_rules! Depcrate_parseextract_fixtures {
() => {
// Module: crate::parse
// Provides: {"extract_fixtures"}
// Dependencies: {}
pub (crate) fn extract_fixtures (item_fn : & mut ItemFn) -> Result < Vec < Fixture > , ErrorsVec > { let mut fixtures_extractor = FixturesFunctionExtractor :: default () ; fixtures_extractor . visit_item_fn_mut (item_fn) ; if fixtures_extractor . 1 . is_empty () { Ok (fixtures_extractor . 0) } else { Err (fixtures_extractor . 1 . into ()) } }
};
}
