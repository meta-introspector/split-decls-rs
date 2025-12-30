// Generated macro for clamp_float_value (function)
macro_rules! Depcrate_mathclamp_float_value {
() => {
// Module: crate::math
// Provides: {"clamp_float_value"}
// Dependencies: {}
# [doc = " Given a floating-point operation and a floating-point value, clamps the result to the output"] # [doc = " range of the given operation according to the C standard, if any."] pub (crate) fn clamp_float_value < S : Semantics > (intrinsic_name : & str , val : IeeeFloat < S > ,) -> IeeeFloat < S > where IeeeFloat < S > : IeeeExt , { let zero = IeeeFloat :: < S > :: ZERO ; let one = IeeeFloat :: < S > :: one () ; let two = IeeeFloat :: < S > :: two () ; let pi = IeeeFloat :: < S > :: pi () ; let pi_over_2 = (pi / two) . value ; match intrinsic_name { # [rustfmt :: skip] | "sinf32" | "sinf64" | "cosf32" | "cosf64" | "tanhf" | "tanh" => val . clamp (one . neg () , one) , "expf32" | "exp2f32" | "expf64" | "exp2f64" => val . maximum (zero) , "coshf" | "cosh" => val . maximum (one) , "acosf" | "acos" => val . clamp (zero , pi) , "asinf" | "asin" => val . clamp (pi . neg () , pi) , "atanf" | "atan" => val . clamp (pi_over_2 . neg () , pi_over_2) , "erff" | "erf" => val . clamp (one . neg () , one) , "erfcf" | "erfc" => val . clamp (zero , two) , "atan2f" | "atan2" => val . clamp (pi . neg () , pi) , _ => val , } }
};
}
