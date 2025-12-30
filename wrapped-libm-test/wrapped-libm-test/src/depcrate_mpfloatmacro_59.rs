// Generated macro for macro_59 (macro)
macro_rules! Depcrate_mpfloatmacro_59 {
() => {
// Module: crate::mpfloat
// Provides: {"macro_59"}
// Dependencies: {}
libm_macros :: for_each_function ! { callback : impl_mp_op , emit_types : [RustFn] , skip : [ceil , ceilf , ceilf128 , ceilf16 , copysign , copysignf , copysignf128 , copysignf16 , fabs , fabsf , fabsf128 , fabsf16 , floor , floorf , floorf128 , floorf16 , fmaximum , fmaximumf , fmaximumf128 , fmaximumf16 , fminimum , fminimumf , fminimumf128 , fminimumf16 , fmod , fmodf , fmodf128 , fmodf16 , frexp , frexpf , ilogb , ilogbf , jn , jnf , ldexp , ldexpf , ldexpf128 , ldexpf16 , lgamma_r , lgammaf_r , modf , modff , nextafter , nextafterf , pow , powf , remquo , remquof , rint , rintf , rintf128 , rintf16 , round , roundeven , roundevenf , roundevenf128 , roundevenf16 , roundf , roundf128 , roundf16 , scalbn , scalbnf , scalbnf128 , scalbnf16 , sincos , sincosf , trunc , truncf , truncf128 , truncf16 , yn , ynf ,] , fn_extra : match MACRO_FN_NAME { expm1 | expm1f => exp_m1 , fabs | fabsf => abs , fdim | fdimf | fdimf16 | fdimf128 => positive_diff , fma | fmaf | fmaf128 => mul_add , fmax | fmaxf | fmaxf16 | fmaxf128 | fmaximum_num | fmaximum_numf | fmaximum_numf16 | fmaximum_numf128 => max , fmin | fminf | fminf16 | fminf128 | fminimum_num | fminimum_numf | fminimum_numf16 | fminimum_numf128 => min , lgamma | lgammaf => ln_gamma , log | logf => ln , log1p | log1pf => ln_1p , tgamma | tgammaf => gamma , _ => MACRO_FN_NAME_NORMALIZED } }
};
}
