macro_rules! deps {
    () => {
        UnknownCryptoError!();
        HmacFunction!();
        Hmac!();
    };
}

macro_rules! _function_f {
    () => {
        deps!();
        # [doc = " The F function as described in the RFC."] fn _function_f < Hmac > (salt : & [u8] , iterations : usize , index : u32 , dk_block : & mut [u8] , block_len : usize , u_step : & mut [u8] , hmac : & mut Hmac ,) -> Result < () , UnknownCryptoError > where Hmac : hmac :: HmacFunction , { debug_assert_eq ! (u_step . len () , Hmac :: HASH_FUNC_OUTSIZE) ; hmac . _update (salt) ? ; hmac . _update (& index . to_be_bytes ()) ? ; hmac . _finalize (u_step) ? ; debug_assert ! (block_len <= u_step . len ()) ; dk_block . copy_from_slice (& u_step [.. block_len]) ; if iterations > 1 { for _ in 1 .. iterations { hmac . _reset () ; hmac . _update (u_step) ? ; hmac . _finalize (u_step) ? ; xor_slices ! (u_step , dk_block) ; } } Ok (()) }
    };
}

_function_f!();