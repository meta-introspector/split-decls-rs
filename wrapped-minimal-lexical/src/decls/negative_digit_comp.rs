macro_rules! deps {
    () => {
        Bigint!();
        Float!();
        ExtendedFloat!();
    };
}

macro_rules! negative_digit_comp {
    () => {
        deps!();
        # [doc = " Generate the significant digits with a negative exponent relative to mantissa."] # [doc = ""] # [doc = " This algorithm is quite simple: we have the significant digits `m1 * b^N1`,"] # [doc = " where `m1` is the bigint mantissa, `b` is the radix, and `N1` is the radix"] # [doc = " exponent. We then calculate the theoretical representation of `b+h`, which"] # [doc = " is `m2 * 2^N2`, where `m2` is the bigint mantissa and `N2` is the binary"] # [doc = " exponent. If we had infinite, efficient floating precision, this would be"] # [doc = " equal to `m1 / b^-N1` and then compare it to `m2 * 2^N2`."] # [doc = ""] # [doc = " Since we cannot divide and keep precision, we must multiply the other:"] # [doc = " if we want to do `m1 / b^-N1 >= m2 * 2^N2`, we can do"] # [doc = " `m1 >= m2 * b^-N1 * 2^N2` Going to the decimal case, we can show and example"] # [doc = " and simplify this further: `m1 >= m2 * 2^N2 * 10^-N1`. Since we can remove"] # [doc = " a power-of-two, this is `m1 >= m2 * 2^(N2 - N1) * 5^-N1`. Therefore, if"] # [doc = " `N2 - N1 > 0`, we need have `m1 >= m2 * 2^(N2 - N1) * 5^-N1`, otherwise,"] # [doc = " we have `m1 * 2^(N1 - N2) >= m2 * 5^-N1`, where the resulting exponents"] # [doc = " are all positive."] # [doc = ""] # [doc = " This allows us to compare both floats using integers efficiently"] # [doc = " without any loss of precision."] # [allow (clippy :: comparison_chain)] pub fn negative_digit_comp < F : Float > (bigmant : Bigint , mut fp : ExtendedFloat , exponent : i32 ,) -> ExtendedFloat { debug_assert ! (fp . mant & (1 << 63) != 0) ; let mut real_digits = bigmant ; let real_exp = exponent ; debug_assert ! (real_exp < 0) ; let mut b = fp ; round :: < F , _ > (& mut b , round_down) ; let b = extended_to_float :: < F > (b) ; let theor = bh (b) ; let mut theor_digits = Bigint :: from_u64 (theor . mant) ; let theor_exp = theor . exp ; let binary_exp = theor_exp - real_exp ; let halfradix_exp = - real_exp ; if halfradix_exp != 0 { theor_digits . pow (5 , halfradix_exp as u32) . unwrap () ; } if binary_exp > 0 { theor_digits . pow (2 , binary_exp as u32) . unwrap () ; } else if binary_exp < 0 { real_digits . pow (2 , (- binary_exp) as u32) . unwrap () ; } let ord = real_digits . data . cmp (& theor_digits . data) ; round :: < F , _ > (& mut fp , | f , s | { round_nearest_tie_even (f , s , | is_odd , _ , _ | { match ord { cmp :: Ordering :: Greater => true , cmp :: Ordering :: Less => false , cmp :: Ordering :: Equal if is_odd => true , cmp :: Ordering :: Equal => false , } }) ; }) ; fp }
    };
}

negative_digit_comp!();