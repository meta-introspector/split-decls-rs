// Generated macro for impl_191 (impl)
macro_rules! Depcrate_matcherimpl_191 {
() => {
// Module: crate::matcher
// Provides: {"impl_191"}
// Dependencies: {}
impl MatcherResult { # [doc = " Returns `true` if `self` is [`MatcherResult::Match`], otherwise"] # [doc = " `false`."] pub fn is_match (self) -> bool { matches ! (self , MatcherResult :: Match) } # [doc = " Returns `true` if `self` is [`MatcherResult::NoMatch`], otherwise"] # [doc = " `false`."] pub fn is_no_match (self) -> bool { matches ! (self , MatcherResult :: NoMatch) } }
};
}
