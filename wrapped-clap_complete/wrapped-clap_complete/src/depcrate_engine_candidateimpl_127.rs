// Generated macro for impl_127 (impl)
macro_rules! Depcrate_engine_candidateimpl_127 {
() => {
// Module: crate::engine::candidate
// Provides: {"impl_127"}
// Dependencies: {}
impl < S : Into < OsString > > From < S > for CompletionCandidate { fn from (s : S) -> Self { CompletionCandidate :: new (s . into ()) } }
};
}
