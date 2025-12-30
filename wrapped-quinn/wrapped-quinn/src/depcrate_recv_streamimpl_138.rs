// Generated macro for impl_138 (impl)
macro_rules! Depcrate_recv_streamimpl_138 {
() => {
// Module: crate::recv_stream
// Provides: {"impl_138"}
// Dependencies: {}
impl Future for Read < '_ > { type Output = Result < Option < usize > , ReadError > ; fn poll (self : Pin < & mut Self > , cx : & mut Context) -> Poll < Self :: Output > { let this = self . get_mut () ; ready ! (this . stream . poll_read_buf (cx , & mut this . buf)) ? ; match this . buf . filled () . len () { 0 if this . buf . capacity () != 0 => Poll :: Ready (Ok (None)) , n => Poll :: Ready (Ok (Some (n))) , } } }
};
}
