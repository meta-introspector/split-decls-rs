// Generated macro for I16Bytes (struct)
macro_rules! Depcrate_endianI16Bytes {
() => {
// Module: crate::endian
// Provides: {"I16Bytes"}
// Dependencies: {}
# [doc = " An unaligned `i16` value with an externally specified endianness of type `E`."] # [derive (Default , Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] # [repr (transparent)] pub struct I16Bytes < E : Endian > ([u8 ; 2] , PhantomData < E >) ;
};
}
