// Generated macro for impl_1317 (impl)
macro_rules! Depcrate_stream_try_stream_try_flatten_unorderedimpl_1317 {
() => {
// Module: crate::stream::try_stream::try_flatten_unordered
// Provides: {"impl_1317"}
// Dependencies: {}
impl < T > Single < T > { # [doc = " Constructs new `Single` with the given value."] fn new (val : T) -> Self { Self (Some (val)) } # [doc = " Attempts to take inner item immediately. Will always succeed if the stream isn't terminated."] fn next_immediate (& mut self) -> Option < T > { self . 0 . take () } }
};
}
