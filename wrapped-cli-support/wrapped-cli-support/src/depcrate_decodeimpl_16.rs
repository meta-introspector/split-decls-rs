// Generated macro for impl_16 (impl)
macro_rules! Depcrate_decodeimpl_16 {
() => {
// Module: crate::decode
// Provides: {"impl_16"}
// Dependencies: {}
impl < 'src > Decode < 'src > for LitOrExpr < 'src > { fn decode (data : & mut & 'src [u8]) -> Self { let str = < & 'src str > :: decode (data) ; Self { str } } }
};
}
