// Generated macro for impl_310 (impl)
macro_rules! Depcrate_matchers_contains_matcherimpl_310 {
() => {
// Module: crate::matchers::contains_matcher
// Provides: {"impl_310"}
// Dependencies: {}
impl < InnerMatcherT > ContainsMatcher < InnerMatcherT > { fn count_matches < T : Debug + Copy , ContainerT > (& self , actual : ContainerT) -> usize where ContainerT : IntoIterator < Item = T > , InnerMatcherT : Matcher < T > , { let mut count = 0 ; for v in actual . into_iter () { if self . inner . matches (v) . into () { count += 1 ; } } count } }
};
}
