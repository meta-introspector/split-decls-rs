// Generated macro for impl_171 (impl)
macro_rules! Depcrate_engine_customimpl_171 {
() => {
// Module: crate::engine::custom
// Provides: {"impl_171"}
// Dependencies: {}
impl < F > ValueCompleter for F where F : Fn (& OsStr) -> Vec < CompletionCandidate > + Send + Sync , { fn complete (& self , current : & OsStr) -> Vec < CompletionCandidate > { self (current) } }
};
}
