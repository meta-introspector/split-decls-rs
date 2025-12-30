// Generated macro for Close (trait)
macro_rules! DepcrateClose {
() => {
// Module: crate
// Provides: {"Close"}
// Dependencies: {}
# [doc = " An extension trait for safely dropping I/O writers."] pub trait Close : Write { # [doc = " Drops an I/O writer and closes any resource handle contained"] # [doc = " inside (such as a raw file descriptor). Ensures that I/O errors"] # [doc = " resulting from closing a handle are not ignored. The writer is"] # [doc = " flushed before any handle is closed. If any errors occur during"] # [doc = " flushing or closing the first such error is returned."] fn close (self) -> Result < () > ; }
};
}
