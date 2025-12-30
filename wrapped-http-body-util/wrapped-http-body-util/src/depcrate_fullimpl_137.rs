// Generated macro for impl_137 (impl)
macro_rules! Depcrate_fullimpl_137 {
() => {
// Module: crate::full
// Provides: {"impl_137"}
// Dependencies: {}
impl < D > From < String > for Full < D > where D : Buf + From < String > , { fn from (s : String) -> Self { Full :: new (D :: from (s)) } }
};
}
