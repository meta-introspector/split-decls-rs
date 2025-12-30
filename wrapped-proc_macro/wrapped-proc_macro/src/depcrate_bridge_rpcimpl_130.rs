// Generated macro for impl_130 (impl)
macro_rules! Depcrate_bridge_rpcimpl_130 {
() => {
// Module: crate::bridge::rpc
// Provides: {"impl_130"}
// Dependencies: {}
impl PanicMessage { pub fn as_str (& self) -> Option < & str > { match self { PanicMessage :: StaticStr (s) => Some (s) , PanicMessage :: String (s) => Some (s) , PanicMessage :: Unknown => None , } } }
};
}
