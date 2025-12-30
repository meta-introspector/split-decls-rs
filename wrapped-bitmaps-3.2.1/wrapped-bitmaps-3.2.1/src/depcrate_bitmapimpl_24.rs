// Generated macro for impl_24 (impl)
macro_rules! Depcrate_bitmapimpl_24 {
() => {
// Module: crate::bitmap
// Provides: {"impl_24"}
// Dependencies: {}
impl < const SIZE : usize > BitOr for Bitmap < { SIZE } > where BitsImpl < { SIZE } > : Bits , { type Output = Self ; fn bitor (mut self , rhs : Self) -> Self :: Output { < BitsImpl < SIZE > as Bits > :: Store :: bit_or (& mut self . data , & rhs . data) ; self } }
};
}
