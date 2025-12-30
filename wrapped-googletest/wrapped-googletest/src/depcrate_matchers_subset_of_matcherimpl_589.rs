// Generated macro for impl_589 (impl)
macro_rules! Depcrate_matchers_subset_of_matcherimpl_589 {
() => {
// Module: crate::matchers::subset_of_matcher
// Provides: {"impl_589"}
// Dependencies: {}
impl < ElementT : PartialEq , ExpectedT > SubsetOfMatcher < ExpectedT > where for < 'a > & 'a ExpectedT : IntoIterator < Item = & 'a ElementT > , { fn expected_is_missing (& self , needle : & ElementT) -> bool { ! self . superset . into_iter () . any (| item | item == needle) } }
};
}
