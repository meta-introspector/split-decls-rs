// Generated macro for as_byte_slice (function)
macro_rules! Depcrate_endianas_byte_slice {
() => {
// Module: crate::endian
// Provides: {"as_byte_slice"}
// Dependencies: {}
pub fn as_byte_slice < E : Encoding < T > , T > (x : & [E]) -> & [u8] { unsafe { core :: slice :: from_raw_parts (x . as_ptr () . cast :: < u8 > () , size_of_val (x)) } }
};
}
