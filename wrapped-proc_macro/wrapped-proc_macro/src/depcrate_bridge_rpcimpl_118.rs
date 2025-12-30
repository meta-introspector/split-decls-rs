// Generated macro for impl_118 (impl)
macro_rules! Depcrate_bridge_rpcimpl_118 {
() => {
// Module: crate::bridge::rpc
// Provides: {"impl_118"}
// Dependencies: {}
impl < 'a , S , A : for < 's > DecodeMut < 'a , 's , S > , B : for < 's > DecodeMut < 'a , 's , S > > DecodeMut < 'a , '_ , S > for (A , B) { fn decode (r : & mut Reader < 'a > , s : & mut S) -> Self { (DecodeMut :: decode (r , s) , DecodeMut :: decode (r , s)) } }
};
}
