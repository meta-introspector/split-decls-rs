// Generated macro for Pattern (struct)
macro_rules! DepcratePattern {
() => {
// Module: crate
// Provides: {"Pattern"}
// Dependencies: {}
# [doc = " A compiled match pattern that can match multipe inputs, or return a"] # [doc = " [`Matcher`] that matches a single input."] # [doc = ""] # [doc = " [`Matcher`]: ../struct.Matcher.html"] # [derive (Debug , Clone)] pub struct Pattern < A = DFA < Vec < u32 > > > { automaton : A , anchored : Anchored , }
};
}
