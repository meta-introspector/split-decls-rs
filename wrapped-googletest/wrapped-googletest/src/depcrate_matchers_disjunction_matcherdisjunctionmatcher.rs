// Generated macro for DisjunctionMatcher (struct)
macro_rules! Depcrate_matchers_disjunction_matcherDisjunctionMatcher {
() => {
// Module: crate::matchers::disjunction_matcher
// Provides: {"DisjunctionMatcher"}
// Dependencies: {}
# [doc = " Matcher created by [`Matcher::or`] and [`any!`]."] # [doc = ""] # [doc = " Both [`Matcher::or`] and [`any!`] nest on m1. In other words,"] # [doc = " both `x.or(y).or(z)` and `any![x, y, z]` produce:"] # [doc = " ```ignore"] # [doc = " DisjunctionMatcher {"] # [doc = "     m1: DisjunctionMatcher {"] # [doc = "         m1: x, m2: y"] # [doc = "     },"] # [doc = "     m2: z"] # [doc = " }"] # [doc = " ```"] # [doc = " **For internal use only. API stablility is not guaranteed!**"] # [doc (hidden)] # [derive (MatcherBase)] pub struct DisjunctionMatcher < M1 , M2 > { m1 : M1 , m2 : M2 , }
};
}
