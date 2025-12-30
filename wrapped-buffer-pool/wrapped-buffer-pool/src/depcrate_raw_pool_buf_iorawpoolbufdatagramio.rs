// Generated macro for RawPoolBufDatagramIo (trait)
macro_rules! Depcrate_raw_pool_buf_ioRawPoolBufDatagramIo {
() => {
// Module: crate::raw_pool_buf_io
// Provides: {"RawPoolBufDatagramIo"}
// Dependencies: {}
pub trait RawPoolBufDatagramIo : Send { fn poll_send_datagrams (& mut self , cx : & mut Context , datagrams : & mut [PooledBuf] ,) -> Poll < io :: Result < usize > > ; fn poll_recv_dgram (& mut self , cx : & mut Context ,) -> Poll < io :: Result < PooledBuf > > ; fn poll_recv_datagrams (& mut self , cx : & mut Context , buffer : & mut Vec < PooledBuf > , limit : usize ,) -> Poll < io :: Result < usize > > { for i in 0 .. limit { match self . poll_recv_dgram (cx) { Poll :: Ready (Ok (buf)) => buffer . push (buf) , Poll :: Ready (Err (err)) => if i > 0 { return Poll :: Ready (Ok (i)) ; } else { return Poll :: Ready (Err (err)) ; } , Poll :: Pending => if i > 0 { return Poll :: Ready (Ok (i)) ; } else { return Poll :: Pending ; } , } } Poll :: Ready (Ok (limit)) } }
};
}
