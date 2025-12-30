// Generated macro for impl_13 (impl)
macro_rules! Depcrate_bitmapimpl_13 {
() => {
// Module: crate::bitmap
// Provides: {"impl_13"}
// Dependencies: {}
impl < const SIZE : usize > Hash for Bitmap < { SIZE } > where BitsImpl < { SIZE } > : Bits , < BitsImpl < { SIZE } > as Bits > :: Store : Hash , { fn hash < H : Hasher > (& self , state : & mut H) { self . as_value () . hash (state) } }
};
}
