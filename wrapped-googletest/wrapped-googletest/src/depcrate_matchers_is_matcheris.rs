// Generated macro for is (function)
macro_rules! Depcrate_matchers_is_matcheris {
() => {
// Module: crate::matchers::is_matcher
// Provides: {"is"}
// Dependencies: {}
# [doc = " Matches precisely values matched by `inner`."] # [doc = ""] # [doc = " The returned matcher produces a description prefixed by the string"] # [doc = " `description`. This is useful in contexts where the test assertion failure"] # [doc = " output must include the additional description."] pub fn is < InnerMatcherT > (description : & str , inner : InnerMatcherT) -> IsMatcher < '_ , InnerMatcherT > { IsMatcher { description , inner } }
};
}
