// Generated macro for impl_296 (impl)
macro_rules! Depcrate_hybrid_errorimpl_296 {
() => {
// Module: crate::hybrid::error
// Provides: {"impl_296"}
// Dependencies: {}
impl BuildError { pub (crate) fn nfa (err : nfa :: thompson :: BuildError) -> BuildError { BuildError { kind : BuildErrorKind :: NFA (err) } } pub (crate) fn insufficient_cache_capacity (minimum : usize , given : usize ,) -> BuildError { BuildError { kind : BuildErrorKind :: InsufficientCacheCapacity { minimum , given } , } } pub (crate) fn insufficient_state_id_capacity (err : LazyStateIDError ,) -> BuildError { BuildError { kind : BuildErrorKind :: InsufficientStateIDCapacity { err } , } } pub (crate) fn unsupported_dfa_word_boundary_unicode () -> BuildError { let msg = "cannot build lazy DFAs for regexes with Unicode word \
                   boundaries; switch to ASCII word boundaries, or \
                   heuristically enable Unicode word boundaries or use a \
                   different regex engine" ; BuildError { kind : BuildErrorKind :: Unsupported (msg) } } }
};
}
