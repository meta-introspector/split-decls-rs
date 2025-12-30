// Generated macro for IncomingBuffer (struct)
macro_rules! Depcrate_endpointIncomingBuffer {
() => {
// Module: crate::endpoint
// Provides: {"IncomingBuffer"}
// Dependencies: {}
# [doc = " Buffered Initial and 0-RTT messages for a pending incoming connection"] # [derive (Default)] struct IncomingBuffer { datagrams : Vec < DatagramConnectionEvent > , total_bytes : u64 , }
};
}
