// Generated macro for IntoConcurrentStream (trait)
macro_rules! Depcrate_concurrent_stream_into_concurrent_streamIntoConcurrentStream {
() => {
// Module: crate::concurrent_stream::into_concurrent_stream
// Provides: {"IntoConcurrentStream"}
// Dependencies: {}
# [doc = " Conversion into a [`ConcurrentStream`]"] pub trait IntoConcurrentStream { # [doc = " The type of the elements being iterated over."] type Item ; # [doc = " Which kind of iterator are we turning this into?"] type IntoConcurrentStream : ConcurrentStream < Item = Self :: Item > ; # [doc = " Convert `self` into a concurrent iterator."] fn into_co_stream (self) -> Self :: IntoConcurrentStream ; }
};
}
