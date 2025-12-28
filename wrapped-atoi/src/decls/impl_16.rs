macro_rules! deps {
    () => {
        FromRadix16Checked!();
        FromRadix16!();
        MaxNumDigits!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl < I > FromRadix16Checked for I where I : Zero + One + FromRadix16 + CheckedMul + CheckedAdd + MaxNumDigits , { fn from_radix_16_checked (text : & [u8]) -> (Option < I > , usize) { let max_safe_digits = max (1 , I :: max_num_digits_negative (nth (10))) - 1 ; let (number , mut index) = I :: from_radix_16 (& text [.. min (text . len () , max_safe_digits)]) ; let mut number = Some (number) ; while index != text . len () { if let Some (digit) = ascii_to_hexdigit (text [index]) { number = number . and_then (| n | n . checked_mul (& nth (16))) ; number = number . and_then (| n | n . checked_add (& digit)) ; index += 1 ; } else { break ; } } (number , index) } }
    };
}

impl_16!()