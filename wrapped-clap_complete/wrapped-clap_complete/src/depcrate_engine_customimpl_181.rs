// Generated macro for impl_181 (impl)
macro_rules! Depcrate_engine_customimpl_181 {
() => {
// Module: crate::engine::custom
// Provides: {"impl_181"}
// Dependencies: {}
impl < F > ValueCandidates for F where F : Fn () -> Vec < CompletionCandidate > + Send + Sync , { fn candidates (& self) -> Vec < CompletionCandidate > { self () } }
};
}
