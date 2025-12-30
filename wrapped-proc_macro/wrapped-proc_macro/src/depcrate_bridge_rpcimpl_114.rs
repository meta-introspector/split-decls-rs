// Generated macro for impl_114 (impl)
macro_rules! Depcrate_bridge_rpcimpl_114 {
() => {
// Module: crate::bridge::rpc
// Provides: {"impl_114"}
// Dependencies: {}
impl < S > DecodeMut < '_ , '_ , S > for char { fn decode (r : & mut Reader < '_ > , s : & mut S) -> Self { char :: from_u32 (u32 :: decode (r , s)) . unwrap () } }
};
}
