// Generated macro for WrappedWaker (struct)
macro_rules! Depcrate_stream_stream_flatten_unorderedWrappedWaker {
() => {
// Module: crate::stream::stream::flatten_unordered
// Provides: {"WrappedWaker"}
// Dependencies: {}
# [doc = " Will update state with the provided value on `wake_by_ref` call"] # [doc = " and then, if there is a need, call `inner_waker`."] struct WrappedWaker { inner_waker : UnsafeCell < Option < Waker > > , poll_state : SharedPollState , need_to_poll : u8 , }
};
}
