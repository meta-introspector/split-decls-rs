// Generated macro for impl_762 (impl)
macro_rules! Depcrate_util_iterimpl_762 {
() => {
// Module: crate::util::iter
// Provides: {"impl_762"}
// Dependencies: {}
impl < 'h , F > Iterator for TryHalfMatchesIter < 'h , F > where F : FnMut (& Input < '_ >) -> Result < Option < HalfMatch > , MatchError > , { type Item = Result < HalfMatch , MatchError > ; # [inline] fn next (& mut self) -> Option < Result < HalfMatch , MatchError > > { self . it . try_advance_half (& mut self . finder) . transpose () } }
};
}
