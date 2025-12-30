// Generated macro for ConjunctionMatcher (struct)
macro_rules! Depcrate_matchers_conjunction_matcherConjunctionMatcher {
() => {
// Module: crate::matchers::conjunction_matcher
// Provides: {"ConjunctionMatcher"}
// Dependencies: {}
# [doc = " Matcher created by [`Matcher::and`] and [`all!`]."] # [doc = ""] # [doc = " Both [`Matcher::and`] and [`all!`] nest on m1. In other words,"] # [doc = " both `x.and(y).and(z)` and `all![x, y, z]` produce:"] # [doc = " ```ignore"] # [doc = " ConjunctionMatcher {"] # [doc = "     m1: ConjunctionMatcher {"] # [doc = "         m1: x,"] # [doc = "         m2: y"] # [doc = "     },"] # [doc = "     m2: z"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " This behavior must be respected"] # [doc = " to ensure that [`Matcher::explain_match`] and [`Matcher::describe`] produce"] # [doc = " useful descriptions."] # [doc = ""] # [doc = " **For internal use only. API stablility is not guaranteed!**"] # [doc (hidden)] # [derive (MatcherBase)] pub struct ConjunctionMatcher < M1 , M2 > { m1 : M1 , m2 : M2 , }
};
}
