// Generated macro for WrapStream (trait)
macro_rules! Depcrate_fut_streamWrapStream {
() => {
// Module: crate::fut::stream
// Provides: {"WrapStream"}
// Dependencies: {}
# [doc = " Helper trait that allows conversion of normal stream into `ActorStream`"] pub trait WrapStream < A > where A : Actor , { # [doc = " The stream that this type can be converted into."] type Stream : ActorStream < A > ; # [deprecated (since = "0.11.0" , note = "Please use WrapStream::into_actor")] # [doc (hidden)] fn actstream (self) -> Self :: Stream ; # [doc = " Convert normal stream to a [`ActorStream`]"] fn into_actor (self , a : & A) -> Self :: Stream ; }
};
}
