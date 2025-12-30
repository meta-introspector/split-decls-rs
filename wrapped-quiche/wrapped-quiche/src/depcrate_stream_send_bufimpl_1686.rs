// Generated macro for impl_1686 (impl)
macro_rules! Depcrate_stream_send_bufimpl_1686 {
() => {
// Module: crate::stream::send_buf
// Provides: {"impl_1686"}
// Dependencies: {}
impl < F : BufFactory > Drop for SendReserve < '_ , F > { fn drop (& mut self) { assert_eq ! (self . reserved , 0) } }
};
}
