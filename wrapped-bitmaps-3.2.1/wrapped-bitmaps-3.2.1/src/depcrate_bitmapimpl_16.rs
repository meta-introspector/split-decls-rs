// Generated macro for impl_16 (impl)
macro_rules! Depcrate_bitmapimpl_16 {
() => {
// Module: crate::bitmap
// Provides: {"impl_16"}
// Dependencies: {}
# [cfg (feature = "std")] impl < const SIZE : usize > Debug for Bitmap < { SIZE } > where BitsImpl < { SIZE } > : Bits , { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , Error > { write ! (f , "{}" , < BitsImpl ::< SIZE > as Bits >:: Store :: to_hex (& self . data)) } }
};
}
