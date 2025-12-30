// Generated macro for impl_26 (impl)
macro_rules! Depcrate_bitmapimpl_26 {
() => {
// Module: crate::bitmap
// Provides: {"impl_26"}
// Dependencies: {}
impl < const SIZE : usize > Not for Bitmap < { SIZE } > where BitsImpl < { SIZE } > : Bits , { type Output = Self ; fn not (mut self) -> Self :: Output { < BitsImpl < SIZE > as Bits > :: Store :: invert (& mut self . data) ; self } }
};
}
