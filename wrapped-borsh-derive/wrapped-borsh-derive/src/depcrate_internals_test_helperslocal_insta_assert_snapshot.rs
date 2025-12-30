// Generated macro for local_insta_assert_snapshot (macro)
macro_rules! Depcrate_internals_test_helperslocal_insta_assert_snapshot {
() => {
// Module: crate::internals::test_helpers
// Provides: {"local_insta_assert_snapshot"}
// Dependencies: {}
macro_rules ! local_insta_assert_snapshot { ($ value : expr) => { { insta :: with_settings ! ({ prepend_module_to_snapshot => false } , { insta :: assert_snapshot ! ($ value) ; }) ; } } ; }
};
}
