// Generated macro for impl_193 (impl)
macro_rules! Depcrate_matcherimpl_193 {
() => {
// Module: crate::matcher
// Provides: {"impl_193"}
// Dependencies: {}
impl < T : Debug + Copy , M : Matcher < T > > Matcher < T > for & M { fn matches (& self , actual : T) -> MatcherResult { (* self) . matches (actual) } fn describe (& self , matcher_result : MatcherResult) -> Description { (* self) . describe (matcher_result) } fn explain_match (& self , actual : T) -> Description { (* self) . explain_match (actual) } }
};
}
