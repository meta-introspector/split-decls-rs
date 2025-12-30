// Generated macro for impl_132 (impl)
macro_rules! Depcrate_bridge_rpcimpl_132 {
() => {
// Module: crate::bridge::rpc
// Provides: {"impl_132"}
// Dependencies: {}
impl < S > DecodeMut < '_ , '_ , S > for PanicMessage { fn decode (r : & mut Reader < '_ > , s : & mut S) -> Self { match Option :: < String > :: decode (r , s) { Some (s) => PanicMessage :: String (s) , None => PanicMessage :: Unknown , } } }
};
}
