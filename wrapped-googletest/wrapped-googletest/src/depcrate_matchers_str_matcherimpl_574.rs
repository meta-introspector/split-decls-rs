// Generated macro for impl_574 (impl)
macro_rules! Depcrate_matchers_str_matcherimpl_574 {
() => {
// Module: crate::matchers::str_matcher
// Provides: {"impl_574"}
// Dependencies: {}
impl < T : Deref < Target = str > > From < EqMatcher < T > > for StrMatcher < T > { fn from (value : EqMatcher < T >) -> Self { Self :: with_default_config (value . expected) } }
};
}
