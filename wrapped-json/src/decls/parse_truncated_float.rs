macro_rules! deps {
    () => {
        Float!();
    };
}

macro_rules! parse_truncated_float {
    () => {
        deps!();
        # [doc = " Parse float from extracted float components."] # [doc = ""] # [doc = " * `integer`     - Slice containing the integer digits."] # [doc = " * `fraction`    - Slice containing the fraction digits."] # [doc = " * `exponent`    - Parsed, 32-bit exponent."] # [doc = ""] # [doc = " Precondition: The integer must not have leading zeros."] pub fn parse_truncated_float < F > (integer : & [u8] , mut fraction : & [u8] , exponent : i32) -> F where F : Float , { while fraction . last () == Some (& b'0') { fraction = & fraction [.. fraction . len () - 1] ; } let mut truncated = 0 ; let mut mantissa : u64 = 0 ; let mut iter = integer . iter () . chain (fraction) ; for & c in & mut iter { mantissa = match add_digit (mantissa , to_digit (c) . unwrap ()) { Some (v) => v , None => { truncated = 1 + iter . count () ; break ; } } ; } let mant_exp = mantissa_exponent (exponent , fraction . len () , truncated) ; let is_truncated = true ; fallback_path (integer , fraction , mantissa , exponent , mant_exp , is_truncated ,) }
    };
}

parse_truncated_float!();