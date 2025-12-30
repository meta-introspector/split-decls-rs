// Generated macro for PatternToken (enum)
macro_rules! DepcratePatternToken {
() => {
// Module: crate
// Provides: {"PatternToken"}
// Dependencies: {}
# [derive (Clone , PartialEq , Eq , PartialOrd , Ord , Hash , Debug)] enum PatternToken { Char (char) , AnyChar , AnySequence , AnyRecursiveSequence , AnyWithin (Vec < CharSpecifier >) , AnyExcept (Vec < CharSpecifier >) , }
};
}
