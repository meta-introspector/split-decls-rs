// Generated macro for impl_133 (impl)
macro_rules! Depcrate_fullimpl_133 {
() => {
// Module: crate::full
// Provides: {"impl_133"}
// Dependencies: {}
impl < D > From < Bytes > for Full < D > where D : Buf + From < Bytes > , { fn from (bytes : Bytes) -> Self { Full :: new (D :: from (bytes)) } }
};
}
