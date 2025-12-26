use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(not(target_os = "redox"))]
impl RecvFlags {
    /// Check if the message contains a truncated datagram.
    ///
    /// This flag is only used for datagram-based sockets,
    /// not for stream sockets.
    ///
    /// On Unix this corresponds to the `MSG_TRUNC` flag.
    /// On Windows this corresponds to the `WSAEMSGSIZE` error code.
    #[cfg(not(target_os = "espidf"))]
    pub const fn is_truncated(self) -> bool {
        self.0 & sys::MSG_TRUNC != 0
    }
}
