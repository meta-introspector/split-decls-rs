// Generated macro for RawSocket (struct)
macro_rules! Depcrate_executor_vsockRawSocket {
() => {
// Module: crate::executor::vsock
// Provides: {"RawSocket"}
// Dependencies: {}
# [derive (Debug)] pub (crate) struct RawSocket { pub remote_cid : u32 , pub remote_port : u32 , pub fwd_cnt : u32 , pub peer_fwd_cnt : u32 , pub peer_buf_alloc : u32 , pub tx_cnt : u32 , pub state : VsockState , pub rx_waker : WakerRegistration , pub tx_waker : WakerRegistration , pub buffer : Vec < u8 > , }
};
}
