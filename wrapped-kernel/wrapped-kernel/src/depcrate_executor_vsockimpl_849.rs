// Generated macro for impl_849 (impl)
macro_rules! Depcrate_executor_vsockimpl_849 {
() => {
// Module: crate::executor::vsock
// Provides: {"impl_849"}
// Dependencies: {}
impl RawSocket { pub fn new (state : VsockState) -> Self { Self { remote_cid : 0 , remote_port : 0 , fwd_cnt : 0 , peer_fwd_cnt : 0 , peer_buf_alloc : 0 , tx_cnt : 0 , state , rx_waker : WakerRegistration :: new () , tx_waker : WakerRegistration :: new () , buffer : Vec :: with_capacity (RAW_SOCKET_BUFFER_SIZE) , } } }
};
}
