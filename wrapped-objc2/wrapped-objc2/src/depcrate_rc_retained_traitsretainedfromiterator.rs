// Generated macro for RetainedFromIterator (trait)
macro_rules! Depcrate_rc_retained_traitsRetainedFromIterator {
() => {
// Module: crate::rc::retained_traits
// Provides: {"RetainedFromIterator"}
// Dependencies: {}
# [doc = " Helper trait to implement [`FromIterator`] on [`Retained`]."] # [doc = ""] # [doc = " This should be implemented in exactly the same fashion as if you were"] # [doc = " implementing `FromIterator` for your type normally."] # [doc (alias = "IdFromIterator")] pub trait RetainedFromIterator < T > : Sized { # [doc = " Creates an `Retained` from an iterator."] fn retained_from_iter < I > (iter : I) -> Retained < Self > where I : IntoIterator < Item = T > ; }
};
}
