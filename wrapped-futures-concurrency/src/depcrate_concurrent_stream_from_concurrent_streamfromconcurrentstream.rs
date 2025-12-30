// Generated macro for FromConcurrentStream (trait)
macro_rules! Depcrate_concurrent_stream_from_concurrent_streamFromConcurrentStream {
() => {
// Module: crate::concurrent_stream::from_concurrent_stream
// Provides: {"FromConcurrentStream"}
// Dependencies: {}
# [doc = " Conversion from a [`ConcurrentStream`]"] # [allow (async_fn_in_trait)] pub trait FromConcurrentStream < A > : Sized { # [doc = " Creates a value from a concurrent iterator."] async fn from_concurrent_stream < T > (iter : T) -> Self where T : IntoConcurrentStream < Item = A > ; }
};
}
