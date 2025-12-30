// Generated macro for unstripped_be_bytes (function)
macro_rules! Depcrate_limbunstripped_be_bytes {
() => {
// Module: crate::limb
// Provides: {"unstripped_be_bytes"}
// Dependencies: {}
# [doc = " Returns an iterator of the big-endian encoding of `limbs`."] # [doc = ""] # [doc = " The number of bytes returned will be a multiple of `LIMB_BYTES`"] # [doc = " and thus may be padded with leading zeros."] pub fn unstripped_be_bytes (limbs : & [Limb]) -> impl ExactSizeIterator < Item = u8 > + Clone + '_ { ArrayFlatMap :: new (limbs . iter () . rev () . copied () , Limb :: to_be_bytes) . unwrap () }
};
}
