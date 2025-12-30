// Generated macro for impl_124 (impl)
macro_rules! Depcrate_bridge_rpcimpl_124 {
() => {
// Module: crate::bridge::rpc
// Provides: {"impl_124"}
// Dependencies: {}
impl < S > DecodeMut < '_ , '_ , S > for String { fn decode (r : & mut Reader < '_ > , s : & mut S) -> Self { < & str > :: decode (r , s) . to_string () } }
};
}
