// Generated macro for impl_119 (impl)
macro_rules! Depcrate_bridge_rpcimpl_119 {
() => {
// Module: crate::bridge::rpc
// Provides: {"impl_119"}
// Dependencies: {}
impl < S > Encode < S > for & [u8] { fn encode (self , w : & mut Writer , s : & mut S) { self . len () . encode (w , s) ; w . write_all (self) . unwrap () ; } }
};
}
