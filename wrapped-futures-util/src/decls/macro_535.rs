macro_rules! deps {
    () => {
        FuturesUnordered!();
        WrappedWaker!();
        SharedPollState!();
    };
}

macro_rules! macro_535 {
    () => {
        deps!();
        pin_project ! { # [doc = " Stream for the [`flatten_unordered`](super::StreamExt::flatten_unordered)"] # [doc = " method with ability to specify flow controller."] # [project = FlattenUnorderedWithFlowControllerProj] # [must_use = "streams do nothing unless polled"] pub struct FlattenUnorderedWithFlowController < St , Fc > where St : Stream { # [pin] inner_streams : FuturesUnordered < PollStreamFut < St :: Item >>, # [pin] stream : St , poll_state : SharedPollState , limit : Option < NonZeroUsize >, is_stream_done : bool , inner_streams_waker : Arc < WrappedWaker >, stream_waker : Arc < WrappedWaker >, flow_controller : PhantomData < Fc > } }
    };
}

macro_535!();