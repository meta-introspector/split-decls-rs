// Generated macro for I32Bytes (struct)
macro_rules! Depcrate_endianI32Bytes {
() => {
// Module: crate::endian
// Provides: {"I32Bytes"}
// Dependencies: {}
# [doc = " An unaligned `i32` value with an externally specified endianness of type `E`."] # [derive (Default , Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] # [repr (transparent)] pub struct I32Bytes < E : Endian > ([u8 ; 4] , PhantomData < E >) ;
};
}
