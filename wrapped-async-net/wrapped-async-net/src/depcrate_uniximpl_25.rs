// Generated macro for impl_25 (impl)
macro_rules! Depcrate_uniximpl_25 {
() => {
// Module: crate::unix
// Provides: {"impl_25"}
// Dependencies: {}
impl Stream for Incoming < '_ > { type Item = io :: Result < UnixStream > ; fn poll_next (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let res = ready ! (Pin :: new (& mut self . incoming) . poll_next (cx)) ; Poll :: Ready (res . map (| res | res . map (| stream | UnixStream :: new (Arc :: new (stream))))) } }
};
}
