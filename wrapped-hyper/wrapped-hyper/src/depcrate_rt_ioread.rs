// Generated macro for Read (trait)
macro_rules! Depcrate_rt_ioRead {
() => {
// Module: crate::rt::io
// Provides: {"Read"}
// Dependencies: {}
# [doc = " Reads bytes from a source."] # [doc = ""] # [doc = " This trait is similar to `std::io::Read`, but supports asynchronous reads."] pub trait Read { # [doc = " Attempts to read bytes into the `buf`."] # [doc = ""] # [doc = " On success, returns `Poll::Ready(Ok(()))` and places data in the"] # [doc = " unfilled portion of `buf`. If no data was read (`buf.remaining()` is"] # [doc = " unchanged), it implies that EOF has been reached."] # [doc = ""] # [doc = " If no data is available for reading, the method returns `Poll::Pending`"] # [doc = " and arranges for the current task (via `cx.waker()`) to receive a"] # [doc = " notification when the object becomes readable or is closed."] fn poll_read (self : Pin < & mut Self > , cx : & mut Context < '_ > , buf : ReadBufCursor < '_ > ,) -> Poll < Result < () , std :: io :: Error > > ; }
};
}
