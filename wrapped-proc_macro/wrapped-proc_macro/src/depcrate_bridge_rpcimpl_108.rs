// Generated macro for impl_108 (impl)
macro_rules! Depcrate_bridge_rpcimpl_108 {
() => {
// Module: crate::bridge::rpc
// Provides: {"impl_108"}
// Dependencies: {}
impl < S > DecodeMut < '_ , '_ , S > for u8 { fn decode (r : & mut Reader < '_ > , _ : & mut S) -> Self { let x = r [0] ; * r = & r [1 ..] ; x } }
};
}
