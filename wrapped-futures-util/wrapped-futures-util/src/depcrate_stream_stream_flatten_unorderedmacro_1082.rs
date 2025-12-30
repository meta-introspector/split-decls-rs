// Generated macro for macro_1082 (macro)
macro_rules! Depcrate_stream_stream_flatten_unorderedmacro_1082 {
() => {
// Module: crate::stream::stream::flatten_unordered
// Provides: {"macro_1082"}
// Dependencies: {}
pin_project ! { # [doc = " Stream for the [`flatten_unordered`](super::StreamExt::flatten_unordered)"] # [doc = " method with ability to specify flow controller."] # [project = FlattenUnorderedWithFlowControllerProj] # [must_use = "streams do nothing unless polled"] pub struct FlattenUnorderedWithFlowController < St , Fc > where St : Stream { # [pin] inner_streams : FuturesUnordered < PollStreamFut < St :: Item >>, # [pin] stream : St , poll_state : SharedPollState , limit : Option < NonZeroUsize >, is_stream_done : bool , inner_streams_waker : Arc < WrappedWaker >, stream_waker : Arc < WrappedWaker >, flow_controller : PhantomData < Fc > } }
};
}
