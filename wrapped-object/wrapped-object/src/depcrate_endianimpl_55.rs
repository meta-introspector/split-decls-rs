// Generated macro for impl_55 (impl)
macro_rules! Depcrate_endianimpl_55 {
() => {
// Module: crate::endian
// Provides: {"impl_55"}
// Dependencies: {}
impl < E : Endian > U32Bytes < E > { # [doc = " Construct a new value given bytes that already have the required endianness."] pub const fn from_bytes (n : [u8 ; 4]) -> Self { Self (n , PhantomData) } # [doc = " Construct a new value given a native endian value."] pub fn new (e : E , n : u32) -> Self { Self (e . write_u32_bytes (n) , PhantomData) } # [doc = " Return the value as a native endian value."] pub fn get (self , e : E) -> u32 { e . read_u32_bytes (self . 0) } # [doc = " Set the value given a native endian value."] pub fn set (& mut self , e : E , n : u32) { self . 0 = e . write_u32_bytes (n) ; } }
};
}
