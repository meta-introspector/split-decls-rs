// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sys/fd/wasi.rs
// Error: expected square brackets
// Problematic line: line 10

use crate::sys::pal::err2io;
use crate::sys_common::{AsInner, AsInnerMut, FromInner, IntoInner};

#[derive(Debug)]
pub struct WasiFd {
    fd: OwnedFd,
}
