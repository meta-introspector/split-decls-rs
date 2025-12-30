// Generated macro for impl_140 (impl)
macro_rules! Depcrate_recv_streamimpl_140 {
() => {
// Module: crate::recv_stream
// Provides: {"impl_140"}
// Dependencies: {}
impl Future for ReadExact < '_ > { type Output = Result < () , ReadExactError > ; fn poll (self : Pin < & mut Self > , cx : & mut Context) -> Poll < Self :: Output > { let this = self . get_mut () ; let mut remaining = this . buf . remaining () ; while remaining > 0 { ready ! (this . stream . poll_read_buf (cx , & mut this . buf)) ? ; let new = this . buf . remaining () ; if new == remaining { return Poll :: Ready (Err (ReadExactError :: FinishedEarly (this . buf . filled () . len ()))) ; } remaining = new ; } Poll :: Ready (Ok (())) } }
};
}
