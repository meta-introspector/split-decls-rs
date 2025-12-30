// Generated macro for macro_722 (macro)
macro_rules! Depcrate_stream_streammacro_722 {
() => {
// Module: crate::stream::stream
// Provides: {"macro_722"}
// Dependencies: {}
# [cfg (feature = "sink")] delegate_all ! (# [doc = " Future for the [`forward`](super::StreamExt::forward) method."] # [cfg_attr (docsrs , doc (cfg (feature = "sink")))] Forward < St , Si > (forward :: Forward < St , Si , St :: Item >) : Debug + Future + FusedFuture + New [| x : St , y : Si | forward :: Forward :: new (x , y)] where St : Stream) ;
};
}
