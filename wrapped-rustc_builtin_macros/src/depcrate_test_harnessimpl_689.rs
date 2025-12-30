// Generated macro for impl_689 (impl)
macro_rules! Depcrate_test_harnessimpl_689 {
() => {
// Module: crate::test_harness
// Provides: {"impl_689"}
// Dependencies: {}
impl < 'a > Visitor < 'a > for InnerItemLinter < '_ > { fn visit_item (& mut self , i : & 'a ast :: Item) { if let Some (attr) = attr :: find_by_name (& i . attrs , sym :: rustc_test_marker) { self . sess . psess . buffer_lint (UNNAMEABLE_TEST_ITEMS , attr . span , i . id , BuiltinLintDiag :: UnnameableTestItems ,) ; } } }
};
}
