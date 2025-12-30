// Generated macro for BuildErrorKind (enum)
macro_rules! Depcrate_meta_errorBuildErrorKind {
() => {
// Module: crate::meta::error
// Provides: {"BuildErrorKind"}
// Dependencies: {}
# [derive (Clone , Debug)] enum BuildErrorKind { Syntax { pid : PatternID , err : regex_syntax :: Error } , NFA (nfa :: thompson :: BuildError) , }
};
}
