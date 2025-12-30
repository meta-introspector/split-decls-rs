// Generated macro for DoubleEndedFallibleStreamingIterator (trait)
macro_rules! DepcrateDoubleEndedFallibleStreamingIterator {
() => {
// Module: crate
// Provides: {"DoubleEndedFallibleStreamingIterator"}
// Dependencies: {}
# [doc = " A fallible, streaming iterator which can be advanced from either end."] pub trait DoubleEndedFallibleStreamingIterator : FallibleStreamingIterator { # [doc = " Advances the state of the iterator to the next item from the end."] # [doc = ""] # [doc = " Iterators start just after the last item, so this method should be called before `get`"] # [doc = " when iterating."] # [doc = ""] # [doc = " The behavior of calling this method after `get` has returned `None`, or after this method"] # [doc = " or `advance` has returned an error is unspecified."] fn advance_back (& mut self) -> Result < () , Self :: Error > ; # [doc = " Advances the back of the iterator, returning the last element."] # [doc = ""] # [doc = " The default implementation simply calls `advance_back` followed by `get`."] # [inline] fn next_back (& mut self) -> Result < Option < & Self :: Item > , Self :: Error > { self . advance_back () ? ; Ok ((* self) . get ()) } }
};
}
