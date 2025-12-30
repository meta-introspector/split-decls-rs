// Generated macro for impl_131 (impl)
macro_rules! Depcrate_bridge_rpcimpl_131 {
() => {
// Module: crate::bridge::rpc
// Provides: {"impl_131"}
// Dependencies: {}
impl < S > Encode < S > for PanicMessage { fn encode (self , w : & mut Writer , s : & mut S) { self . as_str () . encode (w , s) ; } }
};
}
