// Generated macro for U64Bytes (struct)
macro_rules! Depcrate_endianU64Bytes {
() => {
// Module: crate::endian
// Provides: {"U64Bytes"}
// Dependencies: {}
# [doc = " An unaligned `u64` value with an externally specified endianness of type `E`."] # [derive (Default , Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] # [repr (transparent)] pub struct U64Bytes < E : Endian > ([u8 ; 8] , PhantomData < E >) ;
};
}
