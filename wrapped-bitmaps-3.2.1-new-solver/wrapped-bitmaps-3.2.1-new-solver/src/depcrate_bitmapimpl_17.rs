// Generated macro for impl_17 (impl)
macro_rules! Depcrate_bitmapimpl_17 {
() => {
// Module: crate::bitmap
// Provides: {"impl_17"}
// Dependencies: {}
impl < const SIZE : usize > AsRef < [u8] > for Bitmap < { SIZE } > where BitsImpl < { SIZE } > : Bits , { fn as_ref (& self) -> & [u8] { unsafe { core :: slice :: from_raw_parts (& self . data as * const _ as * const u8 , size_of :: < < BitsImpl < SIZE > as Bits > :: Store > () ,) } } }
};
}
