// Generated macro for pending (function)
macro_rules! Depcrate_stream_pendingpending {
() => {
// Module: crate::stream::pending
// Provides: {"pending"}
// Dependencies: {}
# [doc = " Creates a stream which never returns any elements."] # [doc = ""] # [doc = " The returned stream will always return `Pending` when polled."] pub fn pending < T > () -> Pending < T > { assert_stream :: < T , _ > (Pending { _data : marker :: PhantomData }) }
};
}
