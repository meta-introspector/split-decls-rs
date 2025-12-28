macro_rules! deps {
    () => {
        CheckCase!();
        Error!();
    };
}

macro_rules! hex_decode_with_case {
    () => {
        deps!();
        # [doc = " Hex decode src into dst."] # [doc = " The length of src must be even, and it's allowed to decode a zero length src."] # [doc = " The length of dst must be at least src.len() / 2."] # [doc = " when check_case is CheckCase::Lower, the hex string must be lower case."] # [doc = " when check_case is CheckCase::Upper, the hex string must be upper case."] # [doc = " when check_case is CheckCase::None, the hex string can be lower case or upper case."] pub fn hex_decode_with_case (src : & [u8] , dst : & mut [u8] , check_case : CheckCase ,) -> Result < () , Error > { let len = dst . len () . checked_mul (2) . ok_or (Error :: Overflow) ? ; if src . len () < len || ((src . len () & 1) != 0) { return Err (Error :: InvalidLength (len)) ; } if ! hex_check_with_case (src , check_case) { return Err (Error :: InvalidChar) ; } hex_decode_unchecked (src , dst) ; Ok (()) }
    };
}

hex_decode_with_case!();