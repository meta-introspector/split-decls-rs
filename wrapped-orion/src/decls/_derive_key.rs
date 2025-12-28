macro_rules! deps {
    () => {
        Hmac!();
        HmacFunction!();
        UnknownCryptoError!();
    };
}

macro_rules! _derive_key {
    () => {
        deps!();
        # [doc = ""] # [doc = ""] # [doc = " NOTE: Hmac has the output size of the hash function defined,"] # [doc = " but the array initialization with the size cannot depend on a generic parameter,"] # [doc = " because we don't have full support for const generics yet."] fn _derive_key < Hmac , const OUTSIZE : usize > (padded_password : & [u8] , salt : & [u8] , iterations : usize , dest : & mut [u8] ,) -> Result < () , UnknownCryptoError > where Hmac : hmac :: HmacFunction , { debug_assert_eq ! (OUTSIZE , Hmac :: HASH_FUNC_OUTSIZE) ; if dest . is_empty () || iterations < 1 { return Err (UnknownCryptoError) ; } let mut u_step = [0u8 ; OUTSIZE] ; let mut hmac = Hmac :: _new (padded_password) ? ; for (idx , dk_block) in dest . chunks_mut (Hmac :: HASH_FUNC_OUTSIZE) . enumerate () { let block_idx : u32 = 1u32 . checked_add (idx as u32) . unwrap () ; _function_f (salt , iterations , block_idx , dk_block , dk_block . len () , & mut u_step , & mut hmac ,) ? ; hmac . _reset () ; } Ok (()) }
    };
}

_derive_key!();