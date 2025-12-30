// Generated macro for crates_containing_incoherent_inherent_impls (function)
macro_rules! Depcrate_consteval_tests_method_resolutioncrates_containing_incoherent_inherent_impls {
() => {
// Module: crate::consteval::tests::method_resolution
// Provides: {"crates_containing_incoherent_inherent_impls"}
// Dependencies: {}
# [salsa :: tracked (returns (ref))] fn crates_containing_incoherent_inherent_impls (db : & dyn HirDatabase) -> Box < [Crate] > { db . all_crates () . iter () . copied () . filter (| krate | krate . data (db) . origin . is_lang ()) . collect () }
};
}
