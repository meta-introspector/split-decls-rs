// Generated macro for impl_134 (impl)
macro_rules! Depcrate_fullimpl_134 {
() => {
// Module: crate::full
// Provides: {"impl_134"}
// Dependencies: {}
impl < D > From < Vec < u8 > > for Full < D > where D : Buf + From < Vec < u8 > > , { fn from (vec : Vec < u8 >) -> Self { Full :: new (D :: from (vec)) } }
};
}
