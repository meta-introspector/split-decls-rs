// Generated macro for impl_68 (impl)
macro_rules! Depcrate_dfa_denseimpl_68 {
() => {
// Module: crate::dfa::dense
// Provides: {"impl_68"}
// Dependencies: {}
# [cfg (feature = "dfa-build")] impl BuildError { # [doc = " Return the kind of this error."] fn kind (& self) -> & BuildErrorKind { & self . kind } pub (crate) fn nfa (err : thompson :: BuildError) -> BuildError { BuildError { kind : BuildErrorKind :: NFA (err) } } pub (crate) fn unsupported_dfa_word_boundary_unicode () -> BuildError { let msg = "cannot build DFAs for regexes with Unicode word \
                   boundaries; switch to ASCII word boundaries, or \
                   heuristically enable Unicode word boundaries or use a \
                   different regex engine" ; BuildError { kind : BuildErrorKind :: Unsupported (msg) } } pub (crate) fn too_many_states () -> BuildError { BuildError { kind : BuildErrorKind :: TooManyStates } } pub (crate) fn too_many_start_states () -> BuildError { BuildError { kind : BuildErrorKind :: TooManyStartStates } } pub (crate) fn too_many_match_pattern_ids () -> BuildError { BuildError { kind : BuildErrorKind :: TooManyMatchPatternIDs } } pub (crate) fn dfa_exceeded_size_limit (limit : usize) -> BuildError { BuildError { kind : BuildErrorKind :: DFAExceededSizeLimit { limit } } } pub (crate) fn determinize_exceeded_size_limit (limit : usize) -> BuildError { BuildError { kind : BuildErrorKind :: DeterminizeExceededSizeLimit { limit } , } } }
};
}
