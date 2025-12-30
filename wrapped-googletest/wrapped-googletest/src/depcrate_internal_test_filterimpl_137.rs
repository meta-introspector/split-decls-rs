// Generated macro for impl_137 (impl)
macro_rules! Depcrate_internal_test_filterimpl_137 {
() => {
// Module: crate::internal::test_filter
// Provides: {"impl_137"}
// Dependencies: {}
impl TestFilter for Collection { fn filter (& self , test_name : & str) -> bool { (self . positive_equals . iter () . any (| f | f . filter (test_name)) || self . positive_matches . iter () . any (| f | f . filter (test_name))) && (! self . negative_equals . iter () . any (| f | f . filter (test_name))) && (! self . negative_matches . iter () . any (| f | f . filter (test_name))) } }
};
}
