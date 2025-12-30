// Generated macro for impl_769 (impl)
macro_rules! Depcrate_util_iterimpl_769 {
() => {
// Module: crate::util::iter
// Provides: {"impl_769"}
// Dependencies: {}
impl < 'h , F > Iterator for TryMatchesIter < 'h , F > where F : FnMut (& Input < '_ >) -> Result < Option < Match > , MatchError > , { type Item = Result < Match , MatchError > ; # [inline] fn next (& mut self) -> Option < Result < Match , MatchError > > { self . it . try_advance (& mut self . finder) . transpose () } }
};
}
