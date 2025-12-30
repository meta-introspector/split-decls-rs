// Generated macro for Matcher (struct)
macro_rules! DepcrateMatcher {
() => {
// Module: crate
// Provides: {"Matcher"}
// Dependencies: {}
# [doc = " A reference to a [`Pattern`] that matches a single input."] # [doc = ""] # [doc = " [`Pattern`]: ../struct.Pattern.html"] # [derive (Debug , Clone)] pub struct Matcher < A = DFA < Vec < u32 > > > { automaton : A , state : StateID , }
};
}
