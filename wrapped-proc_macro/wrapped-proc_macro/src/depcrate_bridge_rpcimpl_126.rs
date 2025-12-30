// Generated macro for impl_126 (impl)
macro_rules! Depcrate_bridge_rpcimpl_126 {
() => {
// Module: crate::bridge::rpc
// Provides: {"impl_126"}
// Dependencies: {}
impl < 'a , S , T : for < 's > DecodeMut < 'a , 's , S > > DecodeMut < 'a , '_ , S > for Vec < T > { fn decode (r : & mut Reader < 'a > , s : & mut S) -> Self { let len = usize :: decode (r , s) ; let mut vec = Vec :: with_capacity (len) ; for _ in 0 .. len { vec . push (T :: decode (r , s)) ; } vec } }
};
}
