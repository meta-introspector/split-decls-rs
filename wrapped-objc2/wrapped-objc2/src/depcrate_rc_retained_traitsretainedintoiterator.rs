// Generated macro for RetainedIntoIterator (trait)
macro_rules! Depcrate_rc_retained_traitsRetainedIntoIterator {
() => {
// Module: crate::rc::retained_traits
// Provides: {"RetainedIntoIterator"}
// Dependencies: {}
# [doc = " Helper trait to implement [`IntoIterator`] on [`Retained`]."] # [doc = ""] # [doc = " This should be implemented in exactly the same fashion as if you were"] # [doc = " implementing `IntoIterator` for your type normally."] # [doc (alias = "IdIntoIterator")] pub trait RetainedIntoIterator { # [doc = " The type of the elements being iterated over."] type Item ; # [doc = " Which kind of iterator are we turning this into?"] type IntoIter : Iterator < Item = Self :: Item > ; # [doc = " Creates an iterator from an [`Retained`]."] # [doc = ""] # [doc = " You would normally not call this function directly; instead, you'd"] # [doc = " call [`into_iter`](IntoIterator::into_iter) on an [`Retained`]."] fn retained_into_iter (this : Retained < Self >) -> Self :: IntoIter ; }
};
}
