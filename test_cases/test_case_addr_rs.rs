// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/os/net/linux_ext/addr.rs
// Error: expected square brackets
// Problematic line: line 6

use crate::os::unix::net::SocketAddr;
use crate::sealed::Sealed;

/// Platform-specific extensions to [`SocketAddr`].
#[stable(feature = "unix_socket_abstract", since = "1.70.0")]
pub trait SocketAddrExt: Sealed {
    /// Creates a Unix socket address in the abstract namespace.
