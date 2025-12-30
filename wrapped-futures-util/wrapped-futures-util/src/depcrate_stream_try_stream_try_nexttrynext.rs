// Generated macro for TryNext (struct)
macro_rules! Depcrate_stream_try_stream_try_nextTryNext {
() => {
// Module: crate::stream::try_stream::try_next
// Provides: {"TryNext"}
// Dependencies: {}
# [doc = " Future for the [`try_next`](super::TryStreamExt::try_next) method."] # [derive (Debug)] # [must_use = "futures do nothing unless you `.await` or poll them"] pub struct TryNext < 'a , St : ? Sized > { stream : & 'a mut St , }
};
}
