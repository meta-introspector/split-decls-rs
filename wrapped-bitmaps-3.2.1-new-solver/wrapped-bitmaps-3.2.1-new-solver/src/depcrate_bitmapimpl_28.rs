// Generated macro for impl_28 (impl)
macro_rules! Depcrate_bitmapimpl_28 {
() => {
// Module: crate::bitmap
// Provides: {"impl_28"}
// Dependencies: {}
impl < const SIZE : usize > BitOrAssign for Bitmap < { SIZE } > where BitsImpl < { SIZE } > : Bits , { fn bitor_assign (& mut self , rhs : Self) { < BitsImpl < SIZE > as Bits > :: Store :: bit_or (& mut self . data , & rhs . data) ; } }
};
}
