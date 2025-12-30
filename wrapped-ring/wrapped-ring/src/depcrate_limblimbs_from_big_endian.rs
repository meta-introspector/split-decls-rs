// Generated macro for limbs_from_big_endian (function)
macro_rules! Depcrate_limblimbs_from_big_endian {
() => {
// Module: crate::limb
// Provides: {"limbs_from_big_endian"}
// Dependencies: {}
pub fn limbs_from_big_endian < 'a > (input : untrusted :: Input < 'a > , len_bounds : RangeInclusive < usize > ,) -> Result < impl ExactSizeIterator < Item = Limb > + 'a , LenMismatchError > { let r = input . as_slice_less_safe () . rchunks (LIMB_BYTES) . map (| chunk | { let mut padded = [0 ; LIMB_BYTES] ; sliceutil :: overwrite_at_start (& mut padded [(LIMB_BYTES - chunk . len ()) ..] , chunk) ; Limb :: from_be_bytes (padded) }) ; let len = r . len () ; if ! len_bounds . contains (& len) { return Err (LenMismatchError :: new (len)) ; } Ok (r) }
};
}
