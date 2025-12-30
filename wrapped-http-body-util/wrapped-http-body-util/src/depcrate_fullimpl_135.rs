// Generated macro for impl_135 (impl)
macro_rules! Depcrate_fullimpl_135 {
() => {
// Module: crate::full
// Provides: {"impl_135"}
// Dependencies: {}
impl < D > From < & 'static [u8] > for Full < D > where D : Buf + From < & 'static [u8] > , { fn from (slice : & 'static [u8]) -> Self { Full :: new (D :: from (slice)) } }
};
}
