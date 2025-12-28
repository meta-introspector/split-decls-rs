macro_rules! scientific_exponent {
    () => {
        # [inline] pub (crate) fn scientific_exponent (exponent : i32 , integer_digits : usize , fraction_start : usize ,) -> i32 { if integer_digits == 0 { let fraction_start = into_i32 (fraction_start) ; exponent . saturating_sub (fraction_start) . saturating_sub (1) } else { let integer_shift = into_i32 (integer_digits - 1) ; exponent . saturating_add (integer_shift) } }
    };
}

scientific_exponent!();