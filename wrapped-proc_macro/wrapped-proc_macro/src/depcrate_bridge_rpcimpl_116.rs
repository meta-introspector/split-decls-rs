// Generated macro for impl_116 (impl)
macro_rules! Depcrate_bridge_rpcimpl_116 {
() => {
// Module: crate::bridge::rpc
// Provides: {"impl_116"}
// Dependencies: {}
impl < S > DecodeMut < '_ , '_ , S > for NonZero < u32 > { fn decode (r : & mut Reader < '_ > , s : & mut S) -> Self { Self :: new (u32 :: decode (r , s)) . unwrap () } }
};
}
