// Generated macro for impl_14 (impl)
macro_rules! Depcrate_chachaimpl_14 {
() => {
// Module: crate::chacha
// Provides: {"impl_14"}
// Dependencies: {}
impl < T > Clone for Array64 < T > where T : Copy + Default , { fn clone (& self) -> Self { let mut new = Self :: default () ; new . 0 . copy_from_slice (& self . 0) ; new } }
};
}
