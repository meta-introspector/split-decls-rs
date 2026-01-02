mkuse!{use core :: mem :: size_of ;}
mkuse!{use core :: ops ;}
mkuse!{use super :: HalfRep ;}
mkuse!{use crate :: float :: Float ;}
mkuse!{use crate :: int :: { CastFrom , CastInto , DInt , HInt , Int , MinInt } ;}

macro_rules! div_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function div in module {}", module_path!());
    };
}

mkfn!{
    div_introspect!();
    fn div < F : Float > (a : F , b : F) -> F where F :: Int : CastInto < i32 > , F :: Int : From < HalfRep < F > > , F :: Int : From < u8 > , F :: Int : HInt + DInt , < F :: Int as HInt > :: D : ops :: Shr < u32 , Output = < F :: Int as HInt > :: D > , F :: Int : From < u32 > , u16 : CastInto < F :: Int > , i32 : CastInto < F :: Int > , u32 : CastInto < F :: Int > , u128 : CastInto < HalfRep < F > > , { let one = F :: Int :: ONE ; let zero = F :: Int :: ZERO ; let one_hw = HalfRep :: < F > :: ONE ; let zero_hw = HalfRep :: < F > :: ZERO ; let hw = F :: BITS / 2 ; let lo_mask = F :: Int :: MAX >> hw ; let significand_bits = F :: SIG_BITS ; let exponent_sat : F :: Int = F :: EXP_SAT . cast () ; let exponent_bias = F :: EXP_BIAS ; let implicit_bit = F :: IMPLICIT_BIT ; let significand_mask = F :: SIG_MASK ; let sign_bit = F :: SIGN_MASK ; let abs_mask = sign_bit - one ; let exponent_mask = F :: EXP_MASK ; let inf_rep = exponent_mask ; let quiet_bit = implicit_bit >> 1 ; let qnan_rep = exponent_mask | quiet_bit ; let (mut half_iterations , full_iterations) = get_iterations :: < F > () ; let recip_precision = reciprocal_precision :: < F > () ; if F :: BITS == 128 { half_iterations += 1 ; } let a_rep = a . to_bits () ; let b_rep = b . to_bits () ; let a_exponent = (a_rep >> significand_bits) & exponent_sat ; let b_exponent = (b_rep >> significand_bits) & exponent_sat ; let quotient_sign = (a_rep ^ b_rep) & sign_bit ; let mut a_significand = a_rep & significand_mask ; let mut b_significand = b_rep & significand_mask ; let mut res_exponent : i32 = i32 :: cast_from (a_exponent) - i32 :: cast_from (b_exponent) + (exponent_bias as i32) ; if a_exponent . wrapping_sub (one) >= (exponent_sat - one) || b_exponent . wrapping_sub (one) >= (exponent_sat - one) { let a_abs = a_rep & abs_mask ; let b_abs = b_rep & abs_mask ; if a_abs > inf_rep { return F :: from_bits (a_rep | quiet_bit) ; } if b_abs > inf_rep { return F :: from_bits (b_rep | quiet_bit) ; } if a_abs == inf_rep { if b_abs == inf_rep { return F :: from_bits (qnan_rep) ; } else { return F :: from_bits (a_abs | quotient_sign) ; } } if b_abs == inf_rep { return F :: from_bits (quotient_sign) ; } if a_abs == zero { if b_abs == zero { return F :: from_bits (qnan_rep) ; } else { return F :: from_bits (quotient_sign) ; } } if b_abs == zero { return F :: from_bits (inf_rep | quotient_sign) ; } if a_abs < implicit_bit { let (exponent , significand) = F :: normalize (a_significand) ; res_exponent += exponent ; a_significand = significand ; } if b_abs < implicit_bit { let (exponent , significand) = F :: normalize (b_significand) ; res_exponent -= exponent ; b_significand = significand ; } } a_significand |= implicit_bit ; b_significand |= implicit_bit ; let b_uq1 = b_significand << (F :: BITS - significand_bits - 1) ; let mut x_uq0 = if half_iterations > 0 { let b_uq1_hw : HalfRep < F > = b_uq1 . hi () ; let c_hw = c_hw :: < F > () ; debug_assert ! (b_uq1_hw & (one_hw << (HalfRep ::< F >:: BITS - 1)) > zero_hw) ; let mut x_uq0_hw : HalfRep < F > = c_hw . wrapping_sub (b_uq1_hw) ; for _ in 0 .. half_iterations { x_uq0_hw = next_guess (x_uq0_hw , b_uq1_hw) ; } x_uq0_hw = x_uq0_hw . wrapping_sub (one_hw) ; let blo : F :: Int = b_uq1 & lo_mask ; let corr_uq1 : F :: Int = (F :: Int :: from (x_uq0_hw) * F :: Int :: from (b_uq1_hw) + ((F :: Int :: from (x_uq0_hw) * blo) >> hw)) . wrapping_sub (one) . wrapping_neg () ; let lo_corr : F :: Int = corr_uq1 & lo_mask ; let hi_corr : F :: Int = corr_uq1 >> hw ; let mut x_uq0 : F :: Int = ((F :: Int :: from (x_uq0_hw) * hi_corr) << 1u32) . wrapping_add ((F :: Int :: from (x_uq0_hw) * lo_corr) >> (hw - 1)) . wrapping_sub (F :: Int :: from (2u8)) ; x_uq0 -= one ; x_uq0 } else { let c : F :: Int = F :: Int :: from (0x7504F333u32) << (F :: BITS - 32) ; let mut x_uq0 : F :: Int = c . wrapping_sub (b_uq1) ; for _ in 0 .. full_iterations { x_uq0 = next_guess (x_uq0 , b_uq1) ; } x_uq0 } ; x_uq0 = x_uq0 . wrapping_sub (2 . cast ()) ; x_uq0 -= recip_precision . cast () ; let mut quotient_uq1 : F :: Int = x_uq0 . widen_mul (a_significand << 1) . hi () ; let mut residual_lo = if quotient_uq1 < (implicit_bit << 1) { let residual_lo = (a_significand << (significand_bits + 1)) . wrapping_sub (quotient_uq1 . wrapping_mul (b_significand)) ; res_exponent -= 1 ; a_significand <<= 1 ; residual_lo } else { quotient_uq1 >>= 1 ; (a_significand << significand_bits) . wrapping_sub (quotient_uq1 . wrapping_mul (b_significand)) } ; let quotient = quotient_uq1 ; if res_exponent >= i32 :: cast_from (exponent_sat) { return F :: from_bits (inf_rep | quotient_sign) ; } let mut abs_result = if res_exponent > 0 { let mut ret = quotient & significand_mask ; ret |= F :: Int :: from (res_exponent as u32) << significand_bits ; residual_lo <<= 1 ; ret } else { if ((significand_bits as i32) + res_exponent) < 0 { return F :: from_bits (quotient_sign) ; } let ret = quotient . wrapping_shr (u32 :: cast_from (res_exponent . wrapping_neg ()) + 1) ; residual_lo = a_significand . wrapping_shl (significand_bits . wrapping_add (CastInto :: < u32 > :: cast_lossy (res_exponent))) . wrapping_sub (ret . wrapping_mul (b_significand) << 1) ; ret } ; residual_lo += abs_result & one ; abs_result += u8 :: from (residual_lo > b_significand) . into () ; if F :: BITS == 128 || (F :: BITS == 32 && half_iterations > 0) { abs_result += u8 :: from (abs_result < inf_rep && residual_lo > (2 + 1) . cast () * b_significand) . into () ; } if F :: BITS == 128 { abs_result += u8 :: from (abs_result < inf_rep && residual_lo > (4 + 1) . cast () * b_significand) . into () ; } F :: from_bits (abs_result | quotient_sign) }
}

macro_rules! get_iterations_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_iterations in module {}", module_path!());
    };
}

mkfn!{
    get_iterations_introspect!();
    # [doc = " Calculate the number of iterations required for a float type's precision."] # [doc = ""] # [doc = " This returns `(h, f)` where `h` is the number of iterations to be done using integers at half"] # [doc = " the float's bit width, and `f` is the number of iterations done using integers of the float's"] # [doc = " full width. This is further explained in the module documentation."] # [doc = ""] # [doc = " # Requirements"] # [doc = ""] # [doc = " The initial estimate should have at least 8 bits of precision. If this is not true, results"] # [doc = " will be inaccurate."] const fn get_iterations < F : Float > () -> (usize , usize) { let total_iterations = F :: BITS . ilog2 () as usize - 2 ; if 2 * size_of :: < F > () <= size_of :: < * const () > () { (0 , total_iterations) } else { (total_iterations - 1 , 1) } }
}

macro_rules! reciprocal_precision_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function reciprocal_precision in module {}", module_path!());
    };
}

mkfn!{
    reciprocal_precision_introspect!();
    # [doc = " `u_n` for different precisions (with N-1 half-width iterations)."] # [doc = ""] # [doc = " W0 is the precision of C"] # [doc = "   u_0 = (3/4 - 1/sqrt(2) + 2^-W0) * 2^HW"] # [doc = ""] # [doc = " Estimated with bc:"] # [doc = ""] # [doc = " ```text"] # [doc = "   define half1(un) { return 2.0 * (un + un^2) / 2.0^hw + 1.0; }"] # [doc = "   define half2(un) { return 2.0 * un / 2.0^hw + 2.0; }"] # [doc = "   define full1(un) { return 4.0 * (un + 3.01) / 2.0^hw + 2.0 * (un + 3.01)^2 + 4.0; }"] # [doc = "   define full2(un) { return 4.0 * (un + 3.01) / 2.0^hw + 8.0; }"] # [doc = ""] # [doc = "             | f32 (0 + 3) | f32 (2 + 1)  | f64 (3 + 1)  | f128 (4 + 1)"] # [doc = " u_0         | < 184224974 | < 2812.1     | < 184224974  | < 791240234244348797"] # [doc = " u_1         | < 15804007  | < 242.7      | < 15804007   | < 67877681371350440"] # [doc = " u_2         | < 116308    | < 2.81       | < 116308     | < 499533100252317"] # [doc = " u_3         | < 7.31      |              | < 7.31       | < 27054456580"] # [doc = " u_4         |             |              |              | < 80.4"] # [doc = " Final (U_N) | same as u_3 | < 72         | < 218        | < 13920"] # [doc = " ````"] # [doc = ""] # [doc = " Add 2 to `U_N` due to final decrement."] const fn reciprocal_precision < F : Float > () -> u16 { let (half_iterations , full_iterations) = get_iterations :: < F > () ; if full_iterations < 1 { panic ! ("Must have at least one full iteration") ; } if F :: BITS == 32 && half_iterations == 2 && full_iterations == 1 { 74u16 } else if F :: BITS == 32 && half_iterations == 0 && full_iterations == 3 { 10 } else if F :: BITS == 64 && half_iterations == 3 && full_iterations == 1 { 220 } else if F :: BITS == 128 && half_iterations == 4 && full_iterations == 1 { 13922 } else { panic ! ("Invalid number of iterations") } }
}

macro_rules! c_hw_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function c_hw in module {}", module_path!());
    };
}

mkfn!{
    c_hw_introspect!();
    # [doc = " The value of `C` adjusted to half width."] # [doc = ""] # [doc = " C is (3/4 + 1/sqrt(2)) - 1 truncated to W0 fractional bits as UQ0.HW with W0 being either"] # [doc = " 16 or 32 and W0 <= HW. That is, C is the aforementioned 3/4 + 1/sqrt(2) constant (from"] # [doc = " which b/2 is subtracted to obtain x0) wrapped to [0, 1) range."] fn c_hw < F : Float > () -> HalfRep < F > where F :: Int : DInt , u128 : CastInto < HalfRep < F > > , { const C_U128 : u128 = 0x7504f333f9de6108b2fb1366eaa6a542 ; const { C_U128 >> (u128 :: BITS - < HalfRep < F > > :: BITS) } . cast () }
}

macro_rules! next_guess_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function next_guess in module {}", module_path!());
    };
}

mkfn!{
    next_guess_introspect!();
    # [doc = " Perform one iteration at any width to approach `1/b`, given previous guess `x`. Returns"] # [doc = " the next `x` as a UQ0 number."] # [doc = ""] # [doc = " This is the `x_{n+1} = 2*x_n - b*x_n^2` algorithm, implemented as `x_n * (2 - b*x_n)`. It"] # [doc = " uses widening multiplication to calculate the result with necessary precision."] fn next_guess < I > (x_uq0 : I , b_uq1 : I) -> I where I : Int + HInt , < I as HInt > :: D : ops :: Shr < u32 , Output = < I as HInt > :: D > , { let corr_uq1 : I = I :: ZERO . wrapping_sub (x_uq0 . widen_mul (b_uq1) . hi ()) ; (x_uq0 . widen_mul (corr_uq1) >> (I :: BITS - 1)) . lo () }
}
mkitem!{intrinsics ! { # [arm_aeabi_alias = __aeabi_fdiv] pub extern "C" fn __divsf3 (a : f32 , b : f32) -> f32 { div (a , b) } # [arm_aeabi_alias = __aeabi_ddiv] pub extern "C" fn __divdf3 (a : f64 , b : f64) -> f64 { div (a , b) } # [ppc_alias = __divkf3] # [cfg (f128_enabled)] pub extern "C" fn __divtf3 (a : f128 , b : f128) -> f128 { div (a , b) } # [cfg (target_arch = "arm")] pub extern "C" fn __divsf3vfp (a : f32 , b : f32) -> f32 { a / b } # [cfg (target_arch = "arm")] pub extern "C" fn __divdf3vfp (a : f64 , b : f64) -> f64 { a / b } }}