// Generated macro for SocketState (struct)
macro_rules! Depcrate_os_iocpSocketState {
() => {
// Module: crate::os::iocp
// Provides: {"SocketState"}
// Dependencies: {}
# [doc = " Per-socket state."] # [derive (Debug)] struct SocketState { # [doc = " The raw socket handle."] socket : RawSocket , # [doc = " The base socket handle."] base_socket : RawSocket , # [doc = " The event that this socket is currently waiting on."] interest : Event , # [doc = " Whether to listen for error events."] interest_error : bool , # [doc = " The current poll mode."] mode : PollMode , # [doc = " The AFD instance that this socket is registered with."] afd : Arc < Afd < Packet > > , # [doc = " Whether this socket is waiting to be deleted."] waiting_on_delete : bool , # [doc = " The current status of the socket."] status : SocketStatus , }
};
}
