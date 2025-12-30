// Generated macro for _expand_with_parts (function)
macro_rules! Depcrate_hazardous_kdf_hkdf_expand_with_parts {
() => {
// Module: crate::hazardous::kdf::hkdf
// Provides: {"_expand_with_parts"}
// Dependencies: {}
fn _expand_with_parts < Hmac , const OUTSIZE : usize > (prk : & [u8] , info : Option < & [& [u8]] > , dest : & mut [u8] ,) -> Result < () , UnknownCryptoError > where Hmac : hmac :: HmacFunction , { debug_assert_eq ! (OUTSIZE , Hmac :: HASH_FUNC_OUTSIZE) ; debug_assert_eq ! (prk . len () , Hmac :: HASH_FUNC_OUTSIZE) ; if dest . is_empty () || dest . len () > 255 * Hmac :: HASH_FUNC_OUTSIZE { return Err (UnknownCryptoError) ; } let optional_info = info . unwrap_or (& []) ; let mut ctx = Hmac :: _new (prk) ? ; let mut tmp = [0u8 ; OUTSIZE] ; let mut idx : u8 = 1 ; for hlen_block in dest . chunks_mut (Hmac :: HASH_FUNC_OUTSIZE) { for info_part in optional_info . iter () { ctx . _update (info_part) ? ; } ctx . _update (& [idx]) ? ; debug_assert ! (! hlen_block . is_empty () && hlen_block . len () <= Hmac :: HASH_FUNC_OUTSIZE) ; ctx . _finalize (& mut tmp) ? ; hlen_block . copy_from_slice (& tmp [.. hlen_block . len ()]) ; if hlen_block . len () < Hmac :: HASH_FUNC_OUTSIZE { break ; } match idx . checked_add (1) { Some (next) => { idx = next ; ctx . _reset () ; ctx . _update (hlen_block) ? ; } None => break , } ; } tmp . iter_mut () . zeroize () ; Ok (()) }
};
}
