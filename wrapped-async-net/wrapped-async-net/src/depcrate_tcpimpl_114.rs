// Generated macro for impl_114 (impl)
macro_rules! Depcrate_tcpimpl_114 {
() => {
// Module: crate::tcp
// Provides: {"impl_114"}
// Dependencies: {}
impl Stream for Incoming < '_ > { type Item = io :: Result < TcpStream > ; fn poll_next (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let res = ready ! (Pin :: new (& mut self . incoming) . poll_next (cx)) ; Poll :: Ready (res . map (| res | res . map (| stream | TcpStream :: new (Arc :: new (stream))))) } }
};
}
