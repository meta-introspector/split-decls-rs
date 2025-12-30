// Generated macro for impl_1072 (impl)
macro_rules! Depcrate_stream_stream_flatten_unorderedimpl_1072 {
() => {
// Module: crate::stream::stream::flatten_unordered
// Provides: {"impl_1072"}
// Dependencies: {}
impl < 'a , F : FnOnce (& SharedPollState) -> u8 > PollStateBomb < 'a , F > { # [doc = " Constructs new bomb with the given state."] fn new (state : & 'a SharedPollState , drop : F) -> Self { Self { state , drop : Some (drop) } } # [doc = " Deactivates bomb, forces it to not call provided function when dropped."] fn deactivate (mut self) { self . drop . take () ; } }
};
}
