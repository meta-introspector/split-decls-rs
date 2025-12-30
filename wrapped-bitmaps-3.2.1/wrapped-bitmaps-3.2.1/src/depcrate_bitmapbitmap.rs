// Generated macro for Bitmap (struct)
macro_rules! Depcrate_bitmapBitmap {
() => {
// Module: crate::bitmap
// Provides: {"Bitmap"}
// Dependencies: {}
# [doc = " A compact array of bits."] # [doc = ""] # [doc = " The type used to store the bitmap will be the minimum unsigned integer type"] # [doc = " required to fit the number of bits, from `u8` to `u128`. If the size is 1,"] # [doc = " `bool` is used. If the size exceeds 128, an array of `u128` will be used,"] # [doc = " sized as appropriately. The maximum supported size is currently 1024,"] # [doc = " represented by an array `[u128; 8]`."] pub struct Bitmap < const SIZE : usize > where BitsImpl < { SIZE } > : Bits , { pub (crate) data : < BitsImpl < { SIZE } > as Bits > :: Store , }
};
}
