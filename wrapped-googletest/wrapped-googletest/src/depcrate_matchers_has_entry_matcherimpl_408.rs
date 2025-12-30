// Generated macro for impl_408 (impl)
macro_rules! Depcrate_matchers_has_entry_matcherimpl_408 {
() => {
// Module: crate::matchers::has_entry_matcher
// Provides: {"impl_408"}
// Dependencies: {}
impl < 'a , KeyT : Debug + Eq + Hash , ValueT : Debug , MatcherT : Matcher < & 'a ValueT > > Matcher < & 'a HashMap < KeyT , ValueT > > for HasEntryMatcher < KeyT , MatcherT > { fn matches (& self , actual : & 'a HashMap < KeyT , ValueT >) -> MatcherResult { if let Some (value) = actual . get (& self . key) { self . inner . matches (value) } else { MatcherResult :: NoMatch } } fn explain_match (& self , actual : & 'a HashMap < KeyT , ValueT >) -> Description { if let Some (value) = actual . get (& self . key) { format ! ("which contains key {:?}, but is mapped to value {:#?}, {}" , self . key , value , self . inner . explain_match (value)) . into () } else { format ! ("which doesn't contain key {:?}" , self . key) . into () } } fn describe (& self , matcher_result : MatcherResult) -> Description { match matcher_result { MatcherResult :: Match => format ! ("contains key {:?}, which value {}" , self . key , self . inner . describe (MatcherResult :: Match)) . into () , MatcherResult :: NoMatch => format ! ("doesn't contain key {:?} or contains key {:?}, which value {}" , self . key , self . key , self . inner . describe (MatcherResult :: NoMatch)) . into () , } } }
};
}
