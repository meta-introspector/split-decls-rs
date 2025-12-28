macro_rules! deps {
    () => {
        Stream!();
    };
}

macro_rules! FusedStream {
    () => {
        deps!();
        # [doc = " A stream which tracks whether or not the underlying stream"] # [doc = " should no longer be polled."] # [doc = ""] # [doc = " `is_terminated` will return `true` if a future should no longer be polled."] # [doc = " Usually, this state occurs after `poll_next` (or `try_poll_next`) returned"] # [doc = " `Poll::Ready(None)`. However, `is_terminated` may also return `true` if a"] # [doc = " stream has become inactive and can no longer make progress and should be"] # [doc = " ignored or dropped rather than being polled again."] pub trait FusedStream : Stream { # [doc = " Returns `true` if the stream should no longer be polled."] fn is_terminated (& self) -> bool ; }
    };
}

FusedStream!()