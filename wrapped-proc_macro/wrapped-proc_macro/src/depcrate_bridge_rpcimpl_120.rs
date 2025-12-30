// Generated macro for impl_120 (impl)
macro_rules! Depcrate_bridge_rpcimpl_120 {
() => {
// Module: crate::bridge::rpc
// Provides: {"impl_120"}
// Dependencies: {}
impl < 'a , S > DecodeMut < 'a , '_ , S > for & 'a [u8] { fn decode (r : & mut Reader < 'a > , s : & mut S) -> Self { let len = usize :: decode (r , s) ; let xs = & r [.. len] ; * r = & r [len ..] ; xs } }
};
}
