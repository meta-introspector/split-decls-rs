// Generated macro for impl_29 (impl)
macro_rules! Depcrate_bitmapimpl_29 {
() => {
// Module: crate::bitmap
// Provides: {"impl_29"}
// Dependencies: {}
impl < const SIZE : usize > BitXorAssign for Bitmap < { SIZE } > where BitsImpl < { SIZE } > : Bits , { fn bitxor_assign (& mut self , rhs : Self) { < BitsImpl < SIZE > as Bits > :: Store :: bit_xor (& mut self . data , & rhs . data) ; } }
};
}
