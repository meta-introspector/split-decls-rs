// Generated macro for Result (enum)
macro_rules! Depcrate_dfaResult {
() => {
// Module: crate::dfa
// Provides: {"Result"}
// Dependencies: {}
# [doc = " The result of running the DFA."] # [doc = ""] # [doc = " Generally, the result is either a match or not a match, but sometimes the"] # [doc = " DFA runs too slowly because the cache size is too small. In that case, it"] # [doc = " gives up with the intent of falling back to the NFA algorithm."] # [doc = ""] # [doc = " The DFA can also give up if it runs out of room to create new states, or if"] # [doc = " it sees non-ASCII bytes in the presence of a Unicode word boundary."] # [derive (Clone , Debug)] pub enum Result < T > { Match (T) , NoMatch (usize) , Quit , }
};
}
