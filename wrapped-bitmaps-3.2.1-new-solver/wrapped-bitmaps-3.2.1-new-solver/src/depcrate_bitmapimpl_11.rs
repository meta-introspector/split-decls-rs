// Generated macro for impl_11 (impl)
macro_rules! Depcrate_bitmapimpl_11 {
() => {
// Module: crate::bitmap
// Provides: {"impl_11"}
// Dependencies: {}
impl < const SIZE : usize > PartialEq for Bitmap < { SIZE } > where BitsImpl < { SIZE } > : Bits , { fn eq (& self , other : & Self) -> bool { self . data == other . data } }
};
}
