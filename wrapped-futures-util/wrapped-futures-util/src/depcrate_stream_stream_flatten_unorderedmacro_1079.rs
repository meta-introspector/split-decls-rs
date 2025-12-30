// Generated macro for macro_1079 (macro)
macro_rules! Depcrate_stream_stream_flatten_unorderedmacro_1079 {
() => {
// Module: crate::stream::stream::flatten_unordered
// Provides: {"macro_1079"}
// Dependencies: {}
pin_project ! { # [doc = " Future which polls optional inner stream."] # [doc = ""] # [doc = " If it's `Some`, it will attempt to call `poll_next` on it,"] # [doc = " returning `Some((item, next_item_fut))` in case of `Poll::Ready(Some(...))`"] # [doc = " or `None` in case of `Poll::Ready(None)`."] # [doc = ""] # [doc = " If `poll_next` will return `Poll::Pending`, it will be forwarded to"] # [doc = " the future and current task will be notified by waker."] # [must_use = "futures do nothing unless you `.await` or poll them"] struct PollStreamFut < St > { # [pin] stream : Option < St >, } }
};
}
