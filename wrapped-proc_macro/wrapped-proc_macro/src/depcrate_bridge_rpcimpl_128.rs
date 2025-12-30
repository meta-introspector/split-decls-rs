// Generated macro for impl_128 (impl)
macro_rules! Depcrate_bridge_rpcimpl_128 {
() => {
// Module: crate::bridge::rpc
// Provides: {"impl_128"}
// Dependencies: {}
impl From < Box < dyn Any + Send > > for PanicMessage { fn from (payload : Box < dyn Any + Send + 'static >) -> Self { if let Some (s) = payload . downcast_ref :: < & 'static str > () { return PanicMessage :: StaticStr (s) ; } if let Ok (s) = payload . downcast :: < String > () { return PanicMessage :: String (* s) ; } PanicMessage :: Unknown } }
};
}
