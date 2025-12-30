// Generated macro for impl_9 (impl)
macro_rules! Depcrate_collectedimpl_9 {
() => {
// Module: crate::collected
// Provides: {"impl_9"}
// Dependencies: {}
impl < B : Buf > Body for Collected < B > { type Data = B ; type Error = Infallible ; fn poll_frame (mut self : Pin < & mut Self > , _ : & mut Context < '_ > ,) -> Poll < Option < Result < Frame < Self :: Data > , Self :: Error > > > { let frame = if let Some (data) = self . bufs . pop () { Frame :: data (data) } else if let Some (trailers) = self . trailers . take () { Frame :: trailers (trailers) } else { return Poll :: Ready (None) ; } ; Poll :: Ready (Some (Ok (frame))) } }
};
}
