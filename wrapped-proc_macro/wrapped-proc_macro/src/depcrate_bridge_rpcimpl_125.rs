// Generated macro for impl_125 (impl)
macro_rules! Depcrate_bridge_rpcimpl_125 {
() => {
// Module: crate::bridge::rpc
// Provides: {"impl_125"}
// Dependencies: {}
impl < S , T : Encode < S > > Encode < S > for Vec < T > { fn encode (self , w : & mut Writer , s : & mut S) { self . len () . encode (w , s) ; for x in self { x . encode (w , s) ; } } }
};
}
