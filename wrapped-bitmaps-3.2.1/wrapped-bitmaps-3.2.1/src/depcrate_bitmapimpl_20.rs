// Generated macro for impl_20 (impl)
macro_rules! Depcrate_bitmapimpl_20 {
() => {
// Module: crate::bitmap
// Provides: {"impl_20"}
// Dependencies: {}
# [cfg (not (feature = "std"))] impl < const SIZE : usize > Debug for Bitmap < { SIZE } > where BitsImpl < { SIZE } > : Bits , { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , Error > { write ! (f , "Bitmap<{}> {{ ... }}" , SIZE) } }
};
}
