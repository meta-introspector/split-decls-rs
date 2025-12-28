macro_rules! deps {
    () => {
        Number!();
        Float!();
        ExtendedFloat!();
    };
}

macro_rules! compute_float {
    () => {
        deps!();
        # [doc = " Compute a float using an extended-precision representation."] # [doc = ""] # [doc = " Fast conversion of a the significant digits and decimal exponent"] # [doc = " a float to a extended representation with a binary float. This"] # [doc = " algorithm will accurately parse the vast majority of cases,"] # [doc = " and uses a 128-bit representation (with a fallback 192-bit"] # [doc = " representation)."] # [doc = ""] # [doc = " This algorithm scales the exponent by the decimal exponent"] # [doc = " using pre-computed powers-of-5, and calculates if the"] # [doc = " representation can be unambiguously rounded to the nearest"] # [doc = " machine float. Near-halfway cases are not handled here,"] # [doc = " and are represented by a negative, biased binary exponent."] # [doc = ""] # [doc = " The algorithm is described in detail in \"Daniel Lemire, Number Parsing"] # [doc = " at a Gigabyte per Second\" in section 5, \"Fast Algorithm\", and"] # [doc = " section 6, \"Exact Numbers And Ties\", available online:"] # [doc = " <https://arxiv.org/abs/2101.11408.pdf>."] pub fn compute_float < F : Float > (q : i32 , mut w : u64) -> ExtendedFloat { let fp_zero = ExtendedFloat { mant : 0 , exp : 0 , } ; let fp_inf = ExtendedFloat { mant : 0 , exp : F :: INFINITE_POWER , } ; if w == 0 || q < F :: SMALLEST_POWER_OF_TEN { return fp_zero ; } else if q > F :: LARGEST_POWER_OF_TEN { return fp_inf ; } let lz = w . leading_zeros () as i32 ; w <<= lz ; let (lo , hi) = compute_product_approx (q , w , F :: MANTISSA_SIZE as usize + 3) ; if lo == 0xFFFF_FFFF_FFFF_FFFF { let inside_safe_exponent = (q >= - 27) && (q <= 55) ; if ! inside_safe_exponent { return compute_error_scaled :: < F > (q , hi , lz) ; } } let upperbit = (hi >> 63) as i32 ; let mut mantissa = hi >> (upperbit + 64 - F :: MANTISSA_SIZE - 3) ; let mut power2 = power (q) + upperbit - lz - F :: MINIMUM_EXPONENT ; if power2 <= 0 { if - power2 + 1 >= 64 { return fp_zero ; } mantissa >>= - power2 + 1 ; mantissa += mantissa & 1 ; mantissa >>= 1 ; power2 = (mantissa >= (1_u64 << F :: MANTISSA_SIZE)) as i32 ; return ExtendedFloat { mant : mantissa , exp : power2 , } ; } if lo <= 1 && q >= F :: MIN_EXPONENT_ROUND_TO_EVEN && q <= F :: MAX_EXPONENT_ROUND_TO_EVEN && mantissa & 3 == 1 && (mantissa << (upperbit + 64 - F :: MANTISSA_SIZE - 3)) == hi { mantissa &= ! 1_u64 ; } mantissa += mantissa & 1 ; mantissa >>= 1 ; if mantissa >= (2_u64 << F :: MANTISSA_SIZE) { mantissa = 1_u64 << F :: MANTISSA_SIZE ; power2 += 1 ; } mantissa &= ! (1_u64 << F :: MANTISSA_SIZE) ; if power2 >= F :: INFINITE_POWER { return fp_inf ; } ExtendedFloat { mant : mantissa , exp : power2 , } }
    };
}

compute_float!()