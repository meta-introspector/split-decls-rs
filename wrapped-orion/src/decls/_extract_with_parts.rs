macro_rules! deps {
    () => {
        UnknownCryptoError!();
        Hmac!();
        HmacFunction!();
    };
}

macro_rules! _extract_with_parts {
    () => {
        deps!();
        fn _extract_with_parts < Hmac , const OUTSIZE : usize > (salt : & [u8] , ikm : & [& [u8]] ,) -> Result < [u8 ; OUTSIZE] , UnknownCryptoError > where Hmac : hmac :: HmacFunction , { debug_assert_eq ! (OUTSIZE , Hmac :: HASH_FUNC_OUTSIZE) ; let mut dest = [0u8 ; OUTSIZE] ; let mut ctx = Hmac :: _new (salt) ? ; for ikm_part in ikm . iter () { ctx . _update (ikm_part) ? ; } ctx . _finalize (& mut dest) ? ; Ok (dest) }
    };
}

_extract_with_parts!();