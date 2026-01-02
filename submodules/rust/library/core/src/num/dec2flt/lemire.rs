mkuse!{use crate :: num :: dec2flt :: common :: BiasedFp ;}
mkuse!{use crate :: num :: dec2flt :: float :: RawFloat ;}
mkuse!{use crate :: num :: dec2flt :: table :: { LARGEST_POWER_OF_FIVE , POWER_OF_FIVE_128 , SMALLEST_POWER_OF_FIVE , } ;}

macro_rules! compute_float_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function compute_float in module {}", module_path!());
    };
}

mkfn!{
    compute_float_introspect!();
    # [doc = " Compute w * 10^q using an extended-precision float representation."] # [doc = ""] # [doc = " Fast conversion of a the significant digits and decimal exponent"] # [doc = " a float to an extended representation with a binary float. This"] # [doc = " algorithm will accurately parse the vast majority of cases,"] # [doc = " and uses a 128-bit representation (with a fallback 192-bit"] # [doc = " representation)."] # [doc = ""] # [doc = " This algorithm scales the exponent by the decimal exponent"] # [doc = " using pre-computed powers-of-5, and calculates if the"] # [doc = " representation can be unambiguously rounded to the nearest"] # [doc = " machine float. Near-halfway cases are not handled here,"] # [doc = " and are represented by a negative, biased binary exponent."] # [doc = ""] # [doc = " The algorithm is described in detail in \"Daniel Lemire, Number Parsing"] # [doc = " at a Gigabyte per Second\" in section 5, \"Fast Algorithm\", and"] # [doc = " section 6, \"Exact Numbers And Ties\", available online:"] # [doc = " <https://arxiv.org/abs/2101.11408.pdf>."] pub fn compute_float < F : RawFloat > (q : i64 , mut w : u64) -> BiasedFp { let fp_zero = BiasedFp :: zero_pow2 (0) ; let fp_inf = BiasedFp :: zero_pow2 (F :: INFINITE_POWER) ; let fp_error = BiasedFp :: zero_pow2 (- 1) ; if w == 0 || q < F :: SMALLEST_POWER_OF_TEN as i64 { return fp_zero ; } else if q > F :: LARGEST_POWER_OF_TEN as i64 { return fp_inf ; } let lz = w . leading_zeros () ; w <<= lz ; let (lo , hi) = compute_product_approx (q , w , F :: SIG_BITS as usize + 3) ; if lo == 0xFFFF_FFFF_FFFF_FFFF { let inside_safe_exponent = (q >= - 27) && (q <= 55) ; if ! inside_safe_exponent { return fp_error ; } } let upperbit = (hi >> 63) as i32 ; let mut mantissa = hi >> (upperbit + 64 - F :: SIG_BITS as i32 - 3) ; let mut power2 = power (q as i32) + upperbit - lz as i32 - F :: EXP_MIN + 1 ; if power2 <= 0 { if - power2 + 1 >= 64 { return fp_zero ; } mantissa >>= - power2 + 1 ; mantissa += mantissa & 1 ; mantissa >>= 1 ; power2 = (mantissa >= (1_u64 << F :: SIG_BITS)) as i32 ; return BiasedFp { m : mantissa , p_biased : power2 } ; } if lo <= 1 && q >= F :: MIN_EXPONENT_ROUND_TO_EVEN as i64 && q <= F :: MAX_EXPONENT_ROUND_TO_EVEN as i64 && mantissa & 0b11 == 0b01 && (mantissa << (upperbit + 64 - F :: SIG_BITS as i32 - 3)) == hi { mantissa &= ! 1_u64 ; } mantissa += mantissa & 1 ; mantissa >>= 1 ; if mantissa >= (2_u64 << F :: SIG_BITS) { mantissa = 1_u64 << F :: SIG_BITS ; power2 += 1 ; } mantissa &= ! (1_u64 << F :: SIG_BITS) ; if power2 >= F :: INFINITE_POWER { return fp_inf ; } BiasedFp { m : mantissa , p_biased : power2 } }
}

macro_rules! power_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function power in module {}", module_path!());
    };
}

mkfn!{
    power_introspect!();
    # [doc = " Calculate a base 2 exponent from a decimal exponent."] # [doc = " This uses a pre-computed integer approximation for"] # [doc = " log2(10), where 217706 / 2^16 is accurate for the"] # [doc = " entire range of non-finite decimal exponents."] # [inline] fn power (q : i32) -> i32 { (q . wrapping_mul (152_170 + 65536) >> 16) + 63 }
}

macro_rules! full_multiplication_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function full_multiplication in module {}", module_path!());
    };
}

mkfn!{
    full_multiplication_introspect!();
    # [inline] fn full_multiplication (a : u64 , b : u64) -> (u64 , u64) { let r = (a as u128) * (b as u128) ; (r as u64 , (r >> 64) as u64) }
}

macro_rules! compute_product_approx_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function compute_product_approx in module {}", module_path!());
    };
}

mkfn!{
    compute_product_approx_introspect!();
    fn compute_product_approx (q : i64 , w : u64 , precision : usize) -> (u64 , u64) { debug_assert ! (q >= SMALLEST_POWER_OF_FIVE as i64) ; debug_assert ! (q <= LARGEST_POWER_OF_FIVE as i64) ; debug_assert ! (precision <= 64) ; let mask = if precision < 64 { 0xFFFF_FFFF_FFFF_FFFF_u64 >> precision } else { 0xFFFF_FFFF_FFFF_FFFF_u64 } ; let index = (q - SMALLEST_POWER_OF_FIVE as i64) as usize ; let (lo5 , hi5) = POWER_OF_FIVE_128 [index] ; let (mut first_lo , mut first_hi) = full_multiplication (w , lo5) ; if first_hi & mask == mask { let (_ , second_hi) = full_multiplication (w , hi5) ; first_lo = first_lo . wrapping_add (second_hi) ; if second_hi > first_lo { first_hi += 1 ; } } (first_lo , first_hi) }
}