macro_rules! deps {
    () => {
        Hmac!();
        HmacFunction!();
        UnknownCryptoError!();
    };
}

macro_rules! _extract {
    () => {
        deps!();
        # [doc = " The HKDF extract step."] # [doc = ""] # [doc = " NOTE: Hmac has the output size of the hash function defined,"] # [doc = " but the array initialization with the size cannot depend on a generic parameter,"] # [doc = " because we don't have full support for const generics yet."] fn _extract < Hmac , const OUTSIZE : usize > (salt : & [u8] , ikm : & [u8] ,) -> Result < [u8 ; OUTSIZE] , UnknownCryptoError > where Hmac : hmac :: HmacFunction , { debug_assert_eq ! (OUTSIZE , Hmac :: HASH_FUNC_OUTSIZE) ; let mut dest = [0u8 ; OUTSIZE] ; let mut ctx = Hmac :: _new (salt) ? ; ctx . _update (ikm) ? ; ctx . _finalize (& mut dest) ? ; Ok (dest) }
    };
}

_extract!()