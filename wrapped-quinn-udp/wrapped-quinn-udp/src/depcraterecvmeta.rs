// Generated macro for RecvMeta (struct)
macro_rules! DepcrateRecvMeta {
() => {
// Module: crate
// Provides: {"RecvMeta"}
// Dependencies: {}
# [doc = " Metadata for a single buffer filled with bytes received from the network"] # [doc = ""] # [doc = " This associated buffer can contain one or more datagrams, see [`stride`]."] # [doc = ""] # [doc = " [`stride`]: RecvMeta::stride"] # [derive (Debug , Copy , Clone)] # [non_exhaustive] pub struct RecvMeta { # [doc = " The source address of the datagram(s) contained in the buffer"] pub addr : SocketAddr , # [doc = " The number of bytes the associated buffer has"] pub len : usize , # [doc = " The size of a single datagram in the associated buffer"] # [doc = ""] # [doc = " When GRO (Generic Receive Offload) is used this indicates the size of a single"] # [doc = " datagram inside the buffer. If the buffer is larger, that is if [`len`] is greater"] # [doc = " then this value, then the individual datagrams contained have their boundaries at"] # [doc = " `stride` increments from the start. The last datagram could be smaller than"] # [doc = " `stride`."] # [doc = ""] # [doc = " [`len`]: RecvMeta::len"] pub stride : usize , # [doc = " The Explicit Congestion Notification bits for the datagram(s) in the buffer"] pub ecn : Option < EcnCodepoint > , # [doc = " The destination IP address which was encoded in this datagram"] # [doc = ""] # [doc = " Populated on platforms: Windows, Linux, Android (API level > 25),"] # [doc = " FreeBSD, OpenBSD, NetBSD, macOS, and iOS."] pub dst_ip : Option < IpAddr > , # [doc = " The interface index of the interface on which the datagram was received"] pub interface_index : Option < u32 > , }
};
}
