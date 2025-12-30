// Generated macro for impl_23 (impl)
macro_rules! Depcrate_bitmapimpl_23 {
() => {
// Module: crate::bitmap
// Provides: {"impl_23"}
// Dependencies: {}
impl < const SIZE : usize > BitAnd for Bitmap < { SIZE } > where BitsImpl < { SIZE } > : Bits , { type Output = Self ; fn bitand (mut self , rhs : Self) -> Self :: Output { < BitsImpl < SIZE > as Bits > :: Store :: bit_and (& mut self . data , & rhs . data) ; self } }
};
}
