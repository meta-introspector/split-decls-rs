use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Protocol {
    /// Protocol corresponding to `ICMPv4`.
    pub const ICMPV4: Protocol = Protocol(sys::IPPROTO_ICMP);
    /// Protocol corresponding to `ICMPv6`.
    pub const ICMPV6: Protocol = Protocol(sys::IPPROTO_ICMPV6);
    /// Protocol corresponding to `TCP`.
    pub const TCP: Protocol = Protocol(sys::IPPROTO_TCP);
    /// Protocol corresponding to `UDP`.
    pub const UDP: Protocol = Protocol(sys::IPPROTO_UDP);
    #[cfg(target_os = "linux")]
    /// Protocol corresponding to `MPTCP`.
    pub const MPTCP: Protocol = Protocol(sys::IPPROTO_MPTCP);
    /// Protocol corresponding to `DCCP`.
    #[cfg(all(feature = "all", target_os = "linux"))]
    pub const DCCP: Protocol = Protocol(sys::IPPROTO_DCCP);
    /// Protocol corresponding to `SCTP`.
    #[cfg(all(feature = "all", any(target_os = "freebsd", target_os = "linux")))]
    pub const SCTP: Protocol = Protocol(sys::IPPROTO_SCTP);
    /// Protocol corresponding to `UDPLITE`.
    #[cfg(all(
        feature = "all",
        any(
            target_os = "android",
            target_os = "freebsd",
            target_os = "fuchsia",
            target_os = "linux",
        )
    ))]
    pub const UDPLITE: Protocol = Protocol(sys::IPPROTO_UDPLITE);
    /// Protocol corresponding to `DIVERT`.
    #[cfg(all(feature = "all", any(target_os = "freebsd", target_os = "openbsd")))]
    pub const DIVERT: Protocol = Protocol(sys::IPPROTO_DIVERT);
}
