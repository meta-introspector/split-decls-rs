// Generated macro for impl_182 (impl)
macro_rules! Depcrate_channelimpl_182 {
() => {
// Module: crate::channel
// Provides: {"impl_182"}
// Dependencies: {}
impl < D , E > Body for Channel < D , E > where D : Buf , { type Data = D ; type Error = E ; fn poll_frame (self : Pin < & mut Self > , cx : & mut Context < '_ > ,) -> Poll < Option < Result < Frame < Self :: Data > , Self :: Error > > > { let this = self . project () ; match this . rx_frame . poll_recv (cx) { Poll :: Ready (frame @ Some (_)) => return Poll :: Ready (frame . map (Ok)) , Poll :: Ready (None) | Poll :: Pending => { } } use core :: future :: Future ; match this . rx_error . poll (cx) { Poll :: Ready (Ok (error)) => return Poll :: Ready (Some (Err (error))) , Poll :: Ready (Err (_)) => return Poll :: Ready (None) , Poll :: Pending => { } } Poll :: Pending } }
};
}
