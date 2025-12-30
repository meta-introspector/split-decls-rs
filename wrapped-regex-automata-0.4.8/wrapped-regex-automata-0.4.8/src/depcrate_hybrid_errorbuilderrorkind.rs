// Generated macro for BuildErrorKind (enum)
macro_rules! Depcrate_hybrid_errorBuildErrorKind {
() => {
// Module: crate::hybrid::error
// Provides: {"BuildErrorKind"}
// Dependencies: {}
# [derive (Clone , Debug)] enum BuildErrorKind { NFA (nfa :: thompson :: BuildError) , InsufficientCacheCapacity { minimum : usize , given : usize } , InsufficientStateIDCapacity { err : LazyStateIDError } , Unsupported (& 'static str) , }
};
}
