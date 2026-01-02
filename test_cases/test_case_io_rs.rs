// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/os/solid/io.rs
// Error: expected square brackets
// Problematic line: line 54

use crate::sys_common::{AsInner, FromInner, IntoInner};
use crate::{fmt, net, sys};

/// Raw file descriptors.
pub type RawFd = i32;

// The max of this is -2, in two's complement. -1 is `SOLID_NET_INVALID_FD`.
