// Generated macro for impl_327 (impl)
macro_rules! Depcrate_matchers_derefs_to_matcherimpl_327 {
() => {
// Module: crate::matchers::derefs_to_matcher
// Provides: {"impl_327"}
// Dependencies: {}
impl < 'a , ActualT , ExpectedT , Inner > Matcher < & 'a ActualT > for DerefsTo < Inner > where ActualT : Deref < Target = ExpectedT > + Debug , ExpectedT : Copy + Debug + 'a , Inner : Matcher < & 'a ExpectedT > , { fn matches (& self , actual : & 'a ActualT) -> MatcherResult { self . inner . matches (actual . deref ()) } fn describe (& self , matcher_result : MatcherResult) -> Description { self . inner . describe (matcher_result) } fn explain_match (& self , actual : & 'a ActualT) -> Description { self . inner . explain_match (actual . deref ()) } }
};
}
