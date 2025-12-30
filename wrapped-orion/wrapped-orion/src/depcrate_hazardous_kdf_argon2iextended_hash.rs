// Generated macro for extended_hash (function)
macro_rules! Depcrate_hazardous_kdf_argon2iextended_hash {
() => {
// Module: crate::hazardous::kdf::argon2i
// Provides: {"extended_hash"}
// Dependencies: {}
# [doc = " H' as defined in the specification."] fn extended_hash (input : & [u8] , dst : & mut [u8]) -> Result < () , UnknownCryptoError > { if dst . is_empty () { return Err (UnknownCryptoError) ; } let outlen = dst . len () as u32 ; if dst . len () <= BLAKE2B_OUTSIZE { let mut ctx = Blake2b :: new (dst . len ()) ? ; ctx . update (& outlen . to_le_bytes ()) ? ; ctx . update (input) ? ; dst . copy_from_slice (ctx . finalize () ? . as_ref ()) ; } else { let mut ctx = Blake2b :: new (BLAKE2B_OUTSIZE) ? ; ctx . update (& outlen . to_le_bytes ()) ? ; ctx . update (input) ? ; let mut tmp = ctx . finalize () ? ; dst [.. BLAKE2B_OUTSIZE] . copy_from_slice (tmp . as_ref ()) ; let mut pos = BLAKE2B_OUTSIZE / 2 ; let mut toproduce = dst . len () - BLAKE2B_OUTSIZE / 2 ; while toproduce > BLAKE2B_OUTSIZE { ctx . reset () ? ; ctx . update (tmp . as_ref ()) ? ; tmp = ctx . finalize () ? ; dst [pos .. (pos + BLAKE2B_OUTSIZE)] . copy_from_slice (tmp . as_ref ()) ; pos += BLAKE2B_OUTSIZE / 2 ; toproduce -= BLAKE2B_OUTSIZE / 2 ; } ctx = Blake2b :: new (toproduce) ? ; ctx . update (tmp . as_ref ()) ? ; tmp = ctx . finalize () ? ; dst [pos .. outlen as usize] . copy_from_slice (& tmp . as_ref () [.. toproduce]) ; } Ok (()) }
};
}
