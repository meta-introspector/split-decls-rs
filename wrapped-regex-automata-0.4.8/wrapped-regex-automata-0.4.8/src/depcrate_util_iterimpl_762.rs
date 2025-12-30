// Generated macro for impl_762 (impl)
macro_rules! Depcrate_util_iterimpl_762 {
() => {
// Module: crate::util::iter
// Provides: {"impl_762"}
// Dependencies: {}
impl < 'h , F > HalfMatchesIter < 'h , F > { # [doc = " Returns the current `Input` used by this iterator."] # [doc = ""] # [doc = " The `Input` returned is generally equivalent to the one used to"] # [doc = " construct this iterator, but its start position may be different to"] # [doc = " reflect the start of the next search to be executed."] pub fn input < 'i > (& 'i self) -> & 'i Input < 'h > { self . 0 . it . input () } }
};
}
