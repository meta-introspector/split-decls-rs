// Generated macro for impl_8 (impl)
macro_rules! Depcrate_bitmapimpl_8 {
() => {
// Module: crate::bitmap
// Provides: {"impl_8"}
// Dependencies: {}
impl < const SIZE : usize > Clone for Bitmap < { SIZE } > where BitsImpl < { SIZE } > : Bits , { fn clone (& self) -> Self { Bitmap :: from_value (self . data) } }
};
}
