macro_rules! deps {
    () => {
        Float!();
    };
}

macro_rules! parse_concise_float {
    () => {
        deps!();
        # [doc = " Parse float for which the entire integer and fraction parts fit into a 64"] # [doc = " bit mantissa."] pub fn parse_concise_float < F > (mantissa : u64 , mant_exp : i32) -> F where F : Float , { if let Some (float) = fast_path (mantissa , mant_exp) { return float ; } let truncated = false ; let (fp , valid) = moderate_path :: < F > (mantissa , mant_exp , truncated) ; if valid { return fp . into_float :: < F > () ; } let b = fp . into_downward_float :: < F > () ; if b . is_special () { return b ; } let mut buffer = itoa :: Buffer :: new () ; let integer = buffer . format (mantissa) . as_bytes () ; let fraction = & [] ; bhcomp (b , integer , fraction , mant_exp) }
    };
}

parse_concise_float!();