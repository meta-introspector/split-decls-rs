// Generated macro for Connection (struct)
macro_rules! Depcrate_connectionConnection {
() => {
// Module: crate::connection
// Provides: {"Connection"}
// Dependencies: {}
# [doc = " A QUIC connection."] # [doc = ""] # [doc = " If all references to a connection (including every clone of the `Connection` handle, streams of"] # [doc = " incoming streams, and the various stream types) have been dropped, then the connection will be"] # [doc = " automatically closed with an `error_code` of 0 and an empty `reason`. You can also close the"] # [doc = " connection explicitly by calling [`Connection::close()`]."] # [doc = ""] # [doc = " Closing the connection immediately abandons efforts to deliver data to the peer.  Upon"] # [doc = " receiving CONNECTION_CLOSE the peer *may* drop any stream data not yet delivered to the"] # [doc = " application. [`Connection::close()`] describes in more detail how to gracefully close a"] # [doc = " connection without losing application data."] # [doc = ""] # [doc = " May be cloned to obtain another handle to the same connection."] # [doc = ""] # [doc = " [`Connection::close()`]: Connection::close"] # [derive (Debug , Clone)] pub struct Connection (ConnectionRef) ;
};
}
