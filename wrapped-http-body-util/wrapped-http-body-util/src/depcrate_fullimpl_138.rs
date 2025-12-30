// Generated macro for impl_138 (impl)
macro_rules! Depcrate_fullimpl_138 {
() => {
// Module: crate::full
// Provides: {"impl_138"}
// Dependencies: {}
impl < D > From < & 'static str > for Full < D > where D : Buf + From < & 'static str > , { fn from (slice : & 'static str) -> Self { Full :: new (D :: from (slice)) } }
};
}
