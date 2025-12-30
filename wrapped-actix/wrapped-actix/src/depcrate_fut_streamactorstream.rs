// Generated macro for ActorStream (trait)
macro_rules! Depcrate_fut_streamActorStream {
() => {
// Module: crate::fut::stream
// Provides: {"ActorStream"}
// Dependencies: {}
# [doc = " A stream of values, not all of which may have been produced yet."] # [doc = ""] # [doc = " This is similar to `futures_util::stream::Stream` trait, except it works with `Actor`"] pub trait ActorStream < A : Actor > { # [doc = " The type of item this stream will yield on success."] type Item ; fn poll_next (self : Pin < & mut Self > , srv : & mut A , ctx : & mut A :: Context , task : & mut Context < '_ > ,) -> Poll < Option < Self :: Item > > ; }
};
}
