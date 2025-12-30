// Generated macro for impl_143 (impl)
macro_rules! Depcrate_recv_streamimpl_143 {
() => {
// Module: crate::recv_stream
// Provides: {"impl_143"}
// Dependencies: {}
impl Future for ReadChunk < '_ > { type Output = Result < Option < Chunk > , ReadError > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context) -> Poll < Self :: Output > { let (max_length , ordered) = (self . max_length , self . ordered) ; self . stream . poll_read_chunk (cx , max_length , ordered) } }
};
}
