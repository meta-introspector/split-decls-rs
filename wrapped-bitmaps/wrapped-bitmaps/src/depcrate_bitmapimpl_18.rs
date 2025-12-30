// Generated macro for impl_18 (impl)
macro_rules! Depcrate_bitmapimpl_18 {
() => {
// Module: crate::bitmap
// Provides: {"impl_18"}
// Dependencies: {}
impl < const SIZE : usize > AsMut < [u8] > for Bitmap < { SIZE } > where BitsImpl < { SIZE } > : Bits , { fn as_mut (& mut self) -> & mut [u8] { unsafe { core :: slice :: from_raw_parts_mut (& mut self . data as * mut _ as * mut u8 , size_of :: < < BitsImpl < SIZE > as Bits > :: Store > () ,) } } }
};
}
