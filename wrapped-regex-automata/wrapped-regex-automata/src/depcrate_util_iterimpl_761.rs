// Generated macro for impl_761 (impl)
macro_rules! Depcrate_util_iterimpl_761 {
() => {
// Module: crate::util::iter
// Provides: {"impl_761"}
// Dependencies: {}
impl < 'h , F > TryHalfMatchesIter < 'h , F > { # [doc = " Return an infallible version of this iterator."] # [doc = ""] # [doc = " Any item yielded that corresponds to an error results in a panic. This"] # [doc = " is useful if your underlying regex engine is configured in a way that"] # [doc = " it is guaranteed to never return an error."] pub fn infallible (self) -> HalfMatchesIter < 'h , F > { HalfMatchesIter (self) } # [doc = " Returns the current `Input` used by this iterator."] # [doc = ""] # [doc = " The `Input` returned is generally equivalent to the one used to"] # [doc = " construct this iterator, but its start position may be different to"] # [doc = " reflect the start of the next search to be executed."] pub fn input < 'i > (& 'i self) -> & 'i Input < 'h > { self . it . input () } }
};
}
