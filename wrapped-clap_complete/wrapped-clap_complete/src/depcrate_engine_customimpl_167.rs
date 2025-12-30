// Generated macro for impl_167 (impl)
macro_rules! Depcrate_engine_customimpl_167 {
() => {
// Module: crate::engine::custom
// Provides: {"impl_167"}
// Dependencies: {}
impl ArgValueCompleter { # [doc = " Create a new `ArgValueCompleter` with a custom completer"] pub fn new < C > (completer : C) -> Self where C : ValueCompleter + 'static , { Self (Arc :: new (completer)) } # [doc = " Candidates that match `current`"] # [doc = ""] # [doc = " See [`CompletionCandidate`] for more information."] pub fn complete (& self , current : & OsStr) -> Vec < CompletionCandidate > { self . 0 . complete (current) } }
};
}
