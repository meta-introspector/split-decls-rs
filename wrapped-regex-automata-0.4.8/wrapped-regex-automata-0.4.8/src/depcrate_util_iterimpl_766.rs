// Generated macro for impl_766 (impl)
macro_rules! Depcrate_util_iterimpl_766 {
() => {
// Module: crate::util::iter
// Provides: {"impl_766"}
// Dependencies: {}
impl < 'h , F > Iterator for TryMatchesIter < 'h , F > where F : FnMut (& Input < '_ >) -> Result < Option < Match > , MatchError > , { type Item = Result < Match , MatchError > ; # [inline] fn next (& mut self) -> Option < Result < Match , MatchError > > { self . it . try_advance (& mut self . finder) . transpose () } }
};
}
