// Generated macro for impl_106 (impl)
macro_rules! Depcrate_incomingimpl_106 {
() => {
// Module: crate::incoming
// Provides: {"impl_106"}
// Dependencies: {}
impl Future for IncomingFuture { type Output = Result < Connection , ConnectionError > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context) -> Poll < Self :: Output > { match & mut self . 0 { Ok (ref mut connecting) => Pin :: new (connecting) . poll (cx) , Err (e) => Poll :: Ready (Err (e . clone ())) , } } }
};
}
