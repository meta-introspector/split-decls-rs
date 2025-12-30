// Generated macro for SendReserve (struct)
macro_rules! Depcrate_stream_send_bufSendReserve {
() => {
// Module: crate::stream::send_buf
// Provides: {"SendReserve"}
// Dependencies: {}
struct SendReserve < 'a , F : BufFactory > { inner : & 'a mut SendBuf < F > , reserved : usize , fin : bool , }
};
}
