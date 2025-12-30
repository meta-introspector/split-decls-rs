// Generated macro for impl_25 (impl)
macro_rules! Depcrate_bitmapimpl_25 {
() => {
// Module: crate::bitmap
// Provides: {"impl_25"}
// Dependencies: {}
impl < const SIZE : usize > BitXor for Bitmap < { SIZE } > where BitsImpl < { SIZE } > : Bits , { type Output = Self ; fn bitxor (mut self , rhs : Self) -> Self :: Output { < BitsImpl < SIZE > as Bits > :: Store :: bit_xor (& mut self . data , & rhs . data) ; self } }
};
}
