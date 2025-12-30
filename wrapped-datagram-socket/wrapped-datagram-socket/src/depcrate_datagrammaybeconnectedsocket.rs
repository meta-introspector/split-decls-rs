// Generated macro for MaybeConnectedSocket (struct)
macro_rules! Depcrate_datagramMaybeConnectedSocket {
() => {
// Module: crate::datagram
// Provides: {"MaybeConnectedSocket"}
// Dependencies: {}
# [doc = " A cheap wrapper around a datagram socket which describes if it is connected"] # [doc = " to an explicit peer."] # [doc = ""] # [doc = " This struct essentially forwards its underlying socket's `send_to()` method"] # [doc = " to `send()` if the socket is explicitly connected to a peer. This is helpful"] # [doc = " for preventing issues on platforms that do not support `send_to` on"] # [doc = " already-connected sockets."] # [doc = ""] # [doc = " # Warning"] # [doc = " A socket's \"connectedness\" is determined once, when it is created. If the"] # [doc = " socket is created as connected, then later disconnected from its peer, its"] # [doc = " `send_to()` call will fail."] # [doc = ""] # [doc = " For example, MacOS errors if `send_to` is used on a socket that's already"] # [doc = " connected. Only `send` can be used. By using `MaybeConnectedSocket`, you can"] # [doc = " use the same `send` and `send_to` APIs in both client- and server-side code."] # [doc = " Clients, usually with connected sockets, will then forward `send_to` to"] # [doc = " `send`, whereas servers, usually with unconnected sockets, will use"] # [doc = " `send_to`."] # [derive (Clone)] pub struct MaybeConnectedSocket < T > { inner : T , peer : Option < SocketAddr > , }
};
}
