// Generated macro for impl_117 (impl)
macro_rules! Depcrate_bridge_rpcimpl_117 {
() => {
// Module: crate::bridge::rpc
// Provides: {"impl_117"}
// Dependencies: {}
impl < S , A : Encode < S > , B : Encode < S > > Encode < S > for (A , B) { fn encode (self , w : & mut Writer , s : & mut S) { self . 0 . encode (w , s) ; self . 1 . encode (w , s) ; } }
};
}
