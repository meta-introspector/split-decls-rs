// Generated macro for DatagramState (struct)
macro_rules! Depcrate_connection_datagramsDatagramState {
() => {
// Module: crate::connection::datagrams
// Provides: {"DatagramState"}
// Dependencies: {}
# [derive (Default)] pub (super) struct DatagramState { # [doc = " Number of bytes of datagrams that have been received by the local transport but not"] # [doc = " delivered to the application"] pub (super) recv_buffered : usize , pub (super) incoming : VecDeque < Datagram > , pub (super) outgoing : VecDeque < Datagram > , pub (super) outgoing_total : usize , pub (super) send_blocked : bool , }
};
}
