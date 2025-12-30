// Generated macro for Matcher (enum)
macro_rules! Depcrate_literalsMatcher {
() => {
// Module: crate::literals
// Provides: {"Matcher"}
// Dependencies: {}
# [derive (Clone , Debug)] enum Matcher { # [doc = " No literals. (Never advances through the input.)"] Empty , # [doc = " A set of four or more single byte literals."] Bytes (SingleByteSet) , # [doc = " A single substring. (Likely using Boyer-Moore with memchr.)"] Single (SingleSearch) , # [doc = " An Aho-Corasick automaton."] AC (FullAcAutomaton < syntax :: Lit >) , # [doc = " A simd accelerated multiple string matcher."] Teddy128 (Teddy) , }
};
}
