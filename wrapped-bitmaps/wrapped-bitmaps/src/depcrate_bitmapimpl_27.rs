// Generated macro for impl_27 (impl)
macro_rules! Depcrate_bitmapimpl_27 {
() => {
// Module: crate::bitmap
// Provides: {"impl_27"}
// Dependencies: {}
impl < const SIZE : usize > BitAndAssign for Bitmap < { SIZE } > where BitsImpl < { SIZE } > : Bits , { fn bitand_assign (& mut self , rhs : Self) { < BitsImpl < SIZE > as Bits > :: Store :: bit_and (& mut self . data , & rhs . data) ; } }
};
}
