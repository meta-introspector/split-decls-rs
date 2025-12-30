// Generated macro for impl_173 (impl)
macro_rules! Depcrate_engine_customimpl_173 {
() => {
// Module: crate::engine::custom
// Provides: {"impl_173"}
// Dependencies: {}
impl ArgValueCandidates { # [doc = " Create a new `ArgValueCandidates` with a custom completer"] pub fn new < C > (completer : C) -> Self where C : ValueCandidates + 'static , { Self (Arc :: new (completer)) } # [doc = " All potential candidates for an argument."] # [doc = ""] # [doc = " See [`CompletionCandidate`] for more information."] pub fn candidates (& self) -> Vec < CompletionCandidate > { self . 0 . candidates () } }
};
}
