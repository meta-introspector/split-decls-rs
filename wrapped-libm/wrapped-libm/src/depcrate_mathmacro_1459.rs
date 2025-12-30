// Generated macro for macro_1459 (macro)
macro_rules! Depcrate_mathmacro_1459 {
() => {
// Module: crate::math
// Provides: {"macro_1459"}
// Dependencies: {}
cfg_if ! { if # [cfg (f16_enabled)] { pub use self :: ceil :: ceilf16 ; pub use self :: copysign :: copysignf16 ; pub use self :: fabs :: fabsf16 ; pub use self :: fdim :: fdimf16 ; pub use self :: floor :: floorf16 ; pub use self :: fmin_fmax :: { fmaxf16 , fminf16 } ; pub use self :: fminimum_fmaximum :: { fmaximumf16 , fminimumf16 } ; pub use self :: fminimum_fmaximum_num :: { fmaximum_numf16 , fminimum_numf16 } ; pub use self :: fmod :: fmodf16 ; pub use self :: ldexp :: ldexpf16 ; pub use self :: rint :: rintf16 ; pub use self :: round :: roundf16 ; pub use self :: roundeven :: roundevenf16 ; pub use self :: scalbn :: scalbnf16 ; pub use self :: sqrt :: sqrtf16 ; pub use self :: trunc :: truncf16 ; # [allow (unused_imports)] pub (crate) use self :: fma :: fmaf16 ; } }
};
}
