// Generated macro for impl_63 (impl)
macro_rules! Depcrate_endianimpl_63 {
() => {
// Module: crate::endian
// Provides: {"impl_63"}
// Dependencies: {}
impl < E : Endian > I64Bytes < E > { # [doc = " Construct a new value given bytes that already have the required endianness."] pub const fn from_bytes (n : [u8 ; 8]) -> Self { Self (n , PhantomData) } # [doc = " Construct a new value given a native endian value."] pub fn new (e : E , n : i64) -> Self { Self (e . write_i64_bytes (n) , PhantomData) } # [doc = " Return the value as a native endian value."] pub fn get (self , e : E) -> i64 { e . read_i64_bytes (self . 0) } # [doc = " Set the value given a native endian value."] pub fn set (& mut self , e : E , n : i64) { self . 0 = e . write_i64_bytes (n) ; } }
};
}
