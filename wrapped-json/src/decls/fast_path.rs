macro_rules! deps {
    () => {
        Float!();
    };
}

macro_rules! fast_path {
    () => {
        deps!();
        # [doc = " Convert mantissa to exact value for a non-base2 power."] # [doc = ""] # [doc = " Returns the resulting float and if the value can be represented exactly."] pub (crate) fn fast_path < F > (mantissa : u64 , exponent : i32) -> Option < F > where F : Float , { let (min_exp , max_exp) = F :: exponent_limit () ; let shift_exp = F :: mantissa_limit () ; let mantissa_size = F :: MANTISSA_SIZE + 1 ; if mantissa == 0 { Some (F :: ZERO) } else if mantissa >> mantissa_size != 0 { None } else if exponent == 0 { let float = F :: as_cast (mantissa) ; Some (float) } else if exponent >= min_exp && exponent <= max_exp { let float = F :: as_cast (mantissa) ; Some (float . pow10 (exponent)) } else if exponent >= 0 && exponent <= max_exp + shift_exp { let small_powers = POW10_64 ; let shift = exponent - max_exp ; let power = small_powers [shift as usize] ; let value = match mantissa . checked_mul (power) { None => return None , Some (value) => value , } ; if value >> mantissa_size != 0 { None } else { let float = F :: as_cast (value) ; Some (float . pow10 (max_exp)) } } else { None } }
    };
}

fast_path!();