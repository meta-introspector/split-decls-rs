// Generated macro for impl_10 (impl)
macro_rules! Depcrate_bitmapimpl_10 {
() => {
// Module: crate::bitmap
// Provides: {"impl_10"}
// Dependencies: {}
impl < const SIZE : usize > Default for Bitmap < { SIZE } > where BitsImpl < { SIZE } > : Bits , { fn default () -> Self { Bitmap { data : < BitsImpl < SIZE > as Bits > :: Store :: default () , } } }
};
}
