// Generated macro for BuildErrorKind (enum)
macro_rules! Depcrate_dfa_onepassBuildErrorKind {
() => {
// Module: crate::dfa::onepass
// Provides: {"BuildErrorKind"}
// Dependencies: {}
# [doc = " The kind of error that occurred during the construction of a one-pass DFA."] # [derive (Clone , Debug)] enum BuildErrorKind { NFA (crate :: nfa :: thompson :: BuildError) , Word (UnicodeWordBoundaryError) , TooManyStates { limit : u64 } , TooManyPatterns { limit : u64 } , UnsupportedLook { look : Look } , ExceededSizeLimit { limit : usize } , NotOnePass { msg : & 'static str } , }
};
}
