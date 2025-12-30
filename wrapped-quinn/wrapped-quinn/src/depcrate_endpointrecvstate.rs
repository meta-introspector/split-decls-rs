// Generated macro for RecvState (struct)
macro_rules! Depcrate_endpointRecvState {
() => {
// Module: crate::endpoint
// Provides: {"RecvState"}
// Dependencies: {}
# [doc = " State directly involved in handling incoming packets"] struct RecvState { incoming : VecDeque < proto :: Incoming > , connections : ConnectionSet , recv_buf : Box < [u8] > , recv_limiter : WorkLimiter , }
};
}
