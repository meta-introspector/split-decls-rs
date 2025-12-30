// Generated macro for I64Bytes (struct)
macro_rules! Depcrate_endianI64Bytes {
() => {
// Module: crate::endian
// Provides: {"I64Bytes"}
// Dependencies: {}
# [doc = " An unaligned `i64` value with an externally specified endianness of type `E`."] # [derive (Default , Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] # [repr (transparent)] pub struct I64Bytes < E : Endian > ([u8 ; 8] , PhantomData < E >) ;
};
}
