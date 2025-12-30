// Generated macro for impl_115 (impl)
macro_rules! Depcrate_bridge_rpcimpl_115 {
() => {
// Module: crate::bridge::rpc
// Provides: {"impl_115"}
// Dependencies: {}
impl < S > Encode < S > for NonZero < u32 > { fn encode (self , w : & mut Writer , s : & mut S) { self . get () . encode (w , s) ; } }
};
}
