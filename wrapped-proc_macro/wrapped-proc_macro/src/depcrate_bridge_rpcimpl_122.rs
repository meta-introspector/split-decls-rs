// Generated macro for impl_122 (impl)
macro_rules! Depcrate_bridge_rpcimpl_122 {
() => {
// Module: crate::bridge::rpc
// Provides: {"impl_122"}
// Dependencies: {}
impl < 'a , S > DecodeMut < 'a , '_ , S > for & 'a str { fn decode (r : & mut Reader < 'a > , s : & mut S) -> Self { str :: from_utf8 (< & [u8] > :: decode (r , s)) . unwrap () } }
};
}
