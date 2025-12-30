// Generated macro for peel (function)
macro_rules! Depcrate_revision_spec_parse_delegatepeel {
() => {
// Module: crate::revision::spec::parse::delegate
// Provides: {"peel"}
// Dependencies: {}
fn peel (repo : & Repository , obj : & gix_hash :: oid , kind : gix_object :: Kind) -> Result < ObjectId , Error > { let mut obj = repo . find_object (obj) ? ; obj = obj . peel_to_kind (kind) ? ; debug_assert_eq ! (obj . kind , kind , "bug in Object::peel_to_kind() which didn't deliver") ; Ok (obj . id) }
};
}
