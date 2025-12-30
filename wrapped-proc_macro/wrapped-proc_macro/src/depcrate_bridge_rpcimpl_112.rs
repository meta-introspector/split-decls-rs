// Generated macro for impl_112 (impl)
macro_rules! Depcrate_bridge_rpcimpl_112 {
() => {
// Module: crate::bridge::rpc
// Provides: {"impl_112"}
// Dependencies: {}
impl < S > DecodeMut < '_ , '_ , S > for bool { fn decode (r : & mut Reader < '_ > , s : & mut S) -> Self { match u8 :: decode (r , s) { 0 => false , 1 => true , _ => unreachable ! () , } } }
};
}
