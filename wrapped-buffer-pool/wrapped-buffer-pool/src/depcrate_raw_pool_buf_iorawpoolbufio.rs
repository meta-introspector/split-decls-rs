// Generated macro for RawPoolBufIo (trait)
macro_rules! Depcrate_raw_pool_buf_ioRawPoolBufIo {
() => {
// Module: crate::raw_pool_buf_io
// Provides: {"RawPoolBufIo"}
// Dependencies: {}
# [doc = " A trait to optimize read and write operations on pooled buffers."] pub trait RawPoolBufIo : Send { fn poll_send_reserve (& mut self , cx : & mut Context) -> Poll < io :: Result < () > > ; fn send_buf (& mut self , buf : PooledBuf , fin : bool) -> io :: Result < () > ; fn poll_recv_buf (& mut self , cx : & mut Context) -> Poll < io :: Result < PooledBuf > > ; }
};
}
