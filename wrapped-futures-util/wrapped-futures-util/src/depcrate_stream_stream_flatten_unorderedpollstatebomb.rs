// Generated macro for PollStateBomb (struct)
macro_rules! Depcrate_stream_stream_flatten_unorderedPollStateBomb {
() => {
// Module: crate::stream::stream::flatten_unordered
// Provides: {"PollStateBomb"}
// Dependencies: {}
# [doc = " Used to execute some function on the given state when dropped."] struct PollStateBomb < 'a , F : FnOnce (& SharedPollState) -> u8 > { state : & 'a SharedPollState , drop : Option < F > , }
};
}
