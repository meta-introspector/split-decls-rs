// Generated macro for impl_112 (impl)
macro_rules! Depcrate_mpscimpl_112 {
() => {
// Module: crate::mpsc
// Provides: {"impl_112"}
// Dependencies: {}
impl < St : ? Sized + Stream + Unpin > Future for Recv < '_ , St > { type Output = Result < St :: Item , RecvError > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { match Pin :: new (& mut self . stream) . poll_next (cx) { Poll :: Ready (Some (msg)) => Poll :: Ready (Ok (msg)) , Poll :: Ready (None) => Poll :: Ready (Err (RecvError)) , Poll :: Pending => Poll :: Pending , } } }
};
}
