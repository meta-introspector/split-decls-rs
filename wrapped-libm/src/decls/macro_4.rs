macro_rules! macro_4 {
    () => {
        # [cfg (f16_enabled)] libm_helper ! { f16 , funcs : { (fn ceil (x : f16) -> (f16) ; => ceilf16) ; (fn copysign (x : f16 , y : f16) -> (f16) ; => copysignf16) ; (fn fabs (x : f16) -> (f16) ; => fabsf16) ; (fn fdim (x : f16 , y : f16) -> (f16) ; => fdimf16) ; (fn floor (x : f16) -> (f16) ; => floorf16) ; (fn fmax (x : f16 , y : f16) -> (f16) ; => fmaxf16) ; (fn fmaximum_num (x : f16 , y : f16) -> (f16) ; => fmaximum_numf16) ; (fn fmaximumf16 (x : f16 , y : f16) -> (f16) ; => fmaximumf16) ; (fn fmin (x : f16 , y : f16) -> (f16) ; => fminf16) ; (fn fminimum (x : f16 , y : f16) -> (f16) ; => fminimumf16) ; (fn fminimum_num (x : f16 , y : f16) -> (f16) ; => fminimum_numf16) ; (fn fmod (x : f16 , y : f16) -> (f16) ; => fmodf16) ; (fn ldexp (x : f16 , n : i32) -> (f16) ; => ldexpf16) ; (fn rint (x : f16) -> (f16) ; => rintf16) ; (fn round (x : f16) -> (f16) ; => roundf16) ; (fn roundeven (x : f16) -> (f16) ; => roundevenf16) ; (fn scalbn (x : f16 , n : i32) -> (f16) ; => scalbnf16) ; (fn sqrtf (x : f16) -> (f16) ; => sqrtf16) ; (fn truncf (x : f16) -> (f16) ; => truncf16) ; } }
    };
}

macro_4!()