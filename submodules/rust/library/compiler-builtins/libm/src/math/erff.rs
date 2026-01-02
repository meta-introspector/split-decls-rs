mkuse!{use super :: { expf , fabsf } ;}
mkitem!{const ERX : f32 = 8.4506291151e-01 ;}
mkitem!{const EFX8 : f32 = 1.0270333290e+00 ;}
mkitem!{const PP0 : f32 = 1.2837916613e-01 ;}
mkitem!{const PP1 : f32 = - 3.2504209876e-01 ;}
mkitem!{const PP2 : f32 = - 2.8481749818e-02 ;}
mkitem!{const PP3 : f32 = - 5.7702702470e-03 ;}
mkitem!{const PP4 : f32 = - 2.3763017452e-05 ;}
mkitem!{const QQ1 : f32 = 3.9791721106e-01 ;}
mkitem!{const QQ2 : f32 = 6.5022252500e-02 ;}
mkitem!{const QQ3 : f32 = 5.0813062117e-03 ;}
mkitem!{const QQ4 : f32 = 1.3249473704e-04 ;}
mkitem!{const QQ5 : f32 = - 3.9602282413e-06 ;}
mkitem!{const PA0 : f32 = - 2.3621185683e-03 ;}
mkitem!{const PA1 : f32 = 4.1485610604e-01 ;}
mkitem!{const PA2 : f32 = - 3.7220788002e-01 ;}
mkitem!{const PA3 : f32 = 3.1834661961e-01 ;}
mkitem!{const PA4 : f32 = - 1.1089469492e-01 ;}
mkitem!{const PA5 : f32 = 3.5478305072e-02 ;}
mkitem!{const PA6 : f32 = - 2.1663755178e-03 ;}
mkitem!{const QA1 : f32 = 1.0642088205e-01 ;}
mkitem!{const QA2 : f32 = 5.4039794207e-01 ;}
mkitem!{const QA3 : f32 = 7.1828655899e-02 ;}
mkitem!{const QA4 : f32 = 1.2617121637e-01 ;}
mkitem!{const QA5 : f32 = 1.3637083583e-02 ;}
mkitem!{const QA6 : f32 = 1.1984500103e-02 ;}
mkitem!{const RA0 : f32 = - 9.8649440333e-03 ;}
mkitem!{const RA1 : f32 = - 6.9385856390e-01 ;}
mkitem!{const RA2 : f32 = - 1.0558626175e+01 ;}
mkitem!{const RA3 : f32 = - 6.2375331879e+01 ;}
mkitem!{const RA4 : f32 = - 1.6239666748e+02 ;}
mkitem!{const RA5 : f32 = - 1.8460508728e+02 ;}
mkitem!{const RA6 : f32 = - 8.1287437439e+01 ;}
mkitem!{const RA7 : f32 = - 9.8143291473e+00 ;}
mkitem!{const SA1 : f32 = 1.9651271820e+01 ;}
mkitem!{const SA2 : f32 = 1.3765776062e+02 ;}
mkitem!{const SA3 : f32 = 4.3456588745e+02 ;}
mkitem!{const SA4 : f32 = 6.4538726807e+02 ;}
mkitem!{const SA5 : f32 = 4.2900814819e+02 ;}
mkitem!{const SA6 : f32 = 1.0863500214e+02 ;}
mkitem!{const SA7 : f32 = 6.5702495575e+00 ;}
mkitem!{const SA8 : f32 = - 6.0424413532e-02 ;}
mkitem!{const RB0 : f32 = - 9.8649431020e-03 ;}
mkitem!{const RB1 : f32 = - 7.9928326607e-01 ;}
mkitem!{const RB2 : f32 = - 1.7757955551e+01 ;}
mkitem!{const RB3 : f32 = - 1.6063638306e+02 ;}
mkitem!{const RB4 : f32 = - 6.3756646729e+02 ;}
mkitem!{const RB5 : f32 = - 1.0250950928e+03 ;}
mkitem!{const RB6 : f32 = - 4.8351919556e+02 ;}
mkitem!{const SB1 : f32 = 3.0338060379e+01 ;}
mkitem!{const SB2 : f32 = 3.2579251099e+02 ;}
mkitem!{const SB3 : f32 = 1.5367296143e+03 ;}
mkitem!{const SB4 : f32 = 3.1998581543e+03 ;}
mkitem!{const SB5 : f32 = 2.5530502930e+03 ;}
mkitem!{const SB6 : f32 = 4.7452853394e+02 ;}
mkitem!{const SB7 : f32 = - 2.2440952301e+01 ;}

macro_rules! erfc1_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function erfc1 in module {}", module_path!());
    };
}

mkfn!{
    erfc1_introspect!();
    fn erfc1 (x : f32) -> f32 { let s : f32 ; let p : f32 ; let q : f32 ; s = fabsf (x) - 1.0 ; p = PA0 + s * (PA1 + s * (PA2 + s * (PA3 + s * (PA4 + s * (PA5 + s * PA6))))) ; q = 1.0 + s * (QA1 + s * (QA2 + s * (QA3 + s * (QA4 + s * (QA5 + s * QA6))))) ; return 1.0 - ERX - p / q ; }
}

macro_rules! erfc2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function erfc2 in module {}", module_path!());
    };
}

mkfn!{
    erfc2_introspect!();
    fn erfc2 (mut ix : u32 , mut x : f32) -> f32 { let s : f32 ; let r : f32 ; let big_s : f32 ; let z : f32 ; if ix < 0x3fa00000 { return erfc1 (x) ; } x = fabsf (x) ; s = 1.0 / (x * x) ; if ix < 0x4036db6d { r = RA0 + s * (RA1 + s * (RA2 + s * (RA3 + s * (RA4 + s * (RA5 + s * (RA6 + s * RA7)))))) ; big_s = 1.0 + s * (SA1 + s * (SA2 + s * (SA3 + s * (SA4 + s * (SA5 + s * (SA6 + s * (SA7 + s * SA8))))))) ; } else { r = RB0 + s * (RB1 + s * (RB2 + s * (RB3 + s * (RB4 + s * (RB5 + s * RB6))))) ; big_s = 1.0 + s * (SB1 + s * (SB2 + s * (SB3 + s * (SB4 + s * (SB5 + s * (SB6 + s * SB7)))))) ; } ix = x . to_bits () ; z = f32 :: from_bits (ix & 0xffffe000) ; expf (- z * z - 0.5625) * expf ((z - x) * (z + x) + r / big_s) / x }
}

macro_rules! erff_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function erff in module {}", module_path!());
    };
}

mkfn!{
    erff_introspect!();
    # [doc = " Error function (f32)"] # [doc = ""] # [doc = " Calculates an approximation to the “error function”, which estimates"] # [doc = " the probability that an observation will fall within x standard"] # [doc = " deviations of the mean (assuming a normal distribution)."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn erff (x : f32) -> f32 { let r : f32 ; let s : f32 ; let z : f32 ; let y : f32 ; let mut ix : u32 ; let sign : usize ; ix = x . to_bits () ; sign = (ix >> 31) as usize ; ix &= 0x7fffffff ; if ix >= 0x7f800000 { return 1.0 - 2.0 * (sign as f32) + 1.0 / x ; } if ix < 0x3f580000 { if ix < 0x31800000 { return 0.125 * (8.0 * x + EFX8 * x) ; } z = x * x ; r = PP0 + z * (PP1 + z * (PP2 + z * (PP3 + z * PP4))) ; s = 1.0 + z * (QQ1 + z * (QQ2 + z * (QQ3 + z * (QQ4 + z * QQ5)))) ; y = r / s ; return x + x * y ; } if ix < 0x40c00000 { y = 1.0 - erfc2 (ix , x) ; } else { let x1p_120 = f32 :: from_bits (0x03800000) ; y = 1.0 - x1p_120 ; } if sign != 0 { - y } else { y } }
}

macro_rules! erfcf_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function erfcf in module {}", module_path!());
    };
}

mkfn!{
    erfcf_introspect!();
    # [doc = " Complementary error function (f32)"] # [doc = ""] # [doc = " Calculates the complementary probability."] # [doc = " Is `1 - erf(x)`. Is computed directly, so that you can use it to avoid"] # [doc = " the loss of precision that would result from subtracting"] # [doc = " large probabilities (on large `x`) from 1."] pub fn erfcf (x : f32) -> f32 { let r : f32 ; let s : f32 ; let z : f32 ; let y : f32 ; let mut ix : u32 ; let sign : usize ; ix = x . to_bits () ; sign = (ix >> 31) as usize ; ix &= 0x7fffffff ; if ix >= 0x7f800000 { return 2.0 * (sign as f32) + 1.0 / x ; } if ix < 0x3f580000 { if ix < 0x23800000 { return 1.0 - x ; } z = x * x ; r = PP0 + z * (PP1 + z * (PP2 + z * (PP3 + z * PP4))) ; s = 1.0 + z * (QQ1 + z * (QQ2 + z * (QQ3 + z * (QQ4 + z * QQ5)))) ; y = r / s ; if sign != 0 || ix < 0x3e800000 { return 1.0 - (x + x * y) ; } return 0.5 - (x - 0.5 + x * y) ; } if ix < 0x41e00000 { if sign != 0 { return 2.0 - erfc2 (ix , x) ; } else { return erfc2 (ix , x) ; } } let x1p_120 = f32 :: from_bits (0x03800000) ; if sign != 0 { 2.0 - x1p_120 } else { x1p_120 * x1p_120 } }
}