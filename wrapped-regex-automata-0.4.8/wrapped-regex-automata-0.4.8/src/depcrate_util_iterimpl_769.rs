// Generated macro for impl_769 (impl)
macro_rules! Depcrate_util_iterimpl_769 {
() => {
// Module: crate::util::iter
// Provides: {"impl_769"}
// Dependencies: {}
impl < 'h , F > MatchesIter < 'h , F > { # [doc = " Returns the current `Input` used by this iterator."] # [doc = ""] # [doc = " The `Input` returned is generally equivalent to the one used to"] # [doc = " construct this iterator, but its start position may be different to"] # [doc = " reflect the start of the next search to be executed."] pub fn input < 'i > (& 'i self) -> & 'i Input < 'h > { self . 0 . it . input () } }
};
}
