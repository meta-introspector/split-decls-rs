// Generated macro for impl_121 (impl)
macro_rules! Depcrate_bridge_rpcimpl_121 {
() => {
// Module: crate::bridge::rpc
// Provides: {"impl_121"}
// Dependencies: {}
impl < S > Encode < S > for & str { fn encode (self , w : & mut Writer , s : & mut S) { self . as_bytes () . encode (w , s) ; } }
};
}
