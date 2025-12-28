macro_rules! deps {
    () => {
        Hmac!();
        UnknownCryptoError!();
        HmacFunction!();
    };
}

macro_rules! _expand {
    () => {
        deps!();
        # [doc = " The HKDF expand step."] fn _expand < Hmac , const OUTSIZE : usize > (prk : & [u8] , info : Option < & [u8] > , dest : & mut [u8] ,) -> Result < () , UnknownCryptoError > where Hmac : hmac :: HmacFunction , { debug_assert_eq ! (OUTSIZE , Hmac :: HASH_FUNC_OUTSIZE) ; debug_assert_eq ! (prk . len () , Hmac :: HASH_FUNC_OUTSIZE) ; if dest . is_empty () || dest . len () > 255 * Hmac :: HASH_FUNC_OUTSIZE { return Err (UnknownCryptoError) ; } let optional_info = info . unwrap_or (& [0u8 ; 0]) ; let mut ctx = Hmac :: _new (prk) ? ; let mut tmp = [0u8 ; OUTSIZE] ; let mut idx : u8 = 1 ; for hlen_block in dest . chunks_mut (Hmac :: HASH_FUNC_OUTSIZE) { ctx . _update (optional_info) ? ; ctx . _update (& [idx]) ? ; debug_assert ! (! hlen_block . is_empty () && hlen_block . len () <= Hmac :: HASH_FUNC_OUTSIZE) ; ctx . _finalize (& mut tmp) ? ; hlen_block . copy_from_slice (& tmp [.. hlen_block . len ()]) ; if hlen_block . len () < Hmac :: HASH_FUNC_OUTSIZE { break ; } match idx . checked_add (1) { Some (next) => { idx = next ; ctx . _reset () ; ctx . _update (hlen_block) ? ; } None => break , } ; } tmp . iter_mut () . zeroize () ; Ok (()) }
    };
}

_expand!()