macro_rules! FusedFuture {
    () => {
        # [doc = " A future which tracks whether or not the underlying future"] # [doc = " should no longer be polled."] # [doc = ""] # [doc = " `is_terminated` will return `true` if a future should no longer be polled."] # [doc = " Usually, this state occurs after `poll` (or `try_poll`) returned"] # [doc = " `Poll::Ready`. However, `is_terminated` may also return `true` if a future"] # [doc = " has become inactive and can no longer make progress and should be ignored"] # [doc = " or dropped rather than being `poll`ed again."] pub trait FusedFuture : Future { # [doc = " Returns `true` if the underlying future should no longer be polled."] fn is_terminated (& self) -> bool ; }
    };
}

FusedFuture!();