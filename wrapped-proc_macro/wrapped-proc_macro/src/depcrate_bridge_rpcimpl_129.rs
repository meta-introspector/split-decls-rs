// Generated macro for impl_129 (impl)
macro_rules! Depcrate_bridge_rpcimpl_129 {
() => {
// Module: crate::bridge::rpc
// Provides: {"impl_129"}
// Dependencies: {}
impl From < PanicMessage > for Box < dyn Any + Send > { fn from (val : PanicMessage) -> Self { match val { PanicMessage :: StaticStr (s) => Box :: new (s) , PanicMessage :: String (s) => Box :: new (s) , PanicMessage :: Unknown => { struct UnknownPanicMessage ; Box :: new (UnknownPanicMessage) } } } }
};
}
