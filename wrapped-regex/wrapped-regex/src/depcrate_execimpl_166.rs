// Generated macro for impl_166 (impl)
macro_rules! Depcrate_execimpl_166 {
() => {
// Module: crate::exec
// Provides: {"impl_166"}
// Dependencies: {}
impl ProgramCacheInner { fn new (ro : & ExecReadOnly) -> Self { ProgramCacheInner { pikevm : pikevm :: Cache :: new (& ro . nfa) , backtrack : backtrack :: Cache :: new (& ro . nfa) , dfa : dfa :: Cache :: new (& ro . dfa) , dfa_reverse : dfa :: Cache :: new (& ro . dfa_reverse) , } } }
};
}
