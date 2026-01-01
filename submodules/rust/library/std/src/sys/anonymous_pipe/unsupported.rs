// SRC: ../rust/library/std/src/sys/anonymous_pipe/unsupported.rs
use crate::io;
pub use crate::sys::pipe::AnonPipe;

#[inline]
pub fn pipe() -> io::Result<(AnonPipe, AnonPipe)> {
    Err(io::Error::UNSUPPORTED_PLATFORM)
}
