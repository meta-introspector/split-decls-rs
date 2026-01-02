// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/os/net/linux_ext/socket.rs
// Error: expected square brackets
// Problematic line: line 8

use crate::sealed::Sealed;
use crate::sys_common::AsInner;

/// Linux-specific functionality for `AF_UNIX` sockets [`UnixDatagram`]
/// and [`UnixStream`].
///
/// [`UnixDatagram`]: net::UnixDatagram
