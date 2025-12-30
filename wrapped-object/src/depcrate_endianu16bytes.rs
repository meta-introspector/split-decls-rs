// Generated macro for U16Bytes (struct)
macro_rules! Depcrate_endianU16Bytes {
() => {
// Module: crate::endian
// Provides: {"U16Bytes"}
// Dependencies: {}
# [doc = " An unaligned `u16` value with an externally specified endianness of type `E`."] # [derive (Default , Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] # [repr (transparent)] pub struct U16Bytes < E : Endian > ([u8 ; 2] , PhantomData < E >) ;
};
}
