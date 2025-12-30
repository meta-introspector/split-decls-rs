// Generated macro for ziggurat (function)
macro_rules! Depcrate_utilsziggurat {
() => {
// Module: crate::utils
// Provides: {"ziggurat"}
// Dependencies: {}
# [doc = " Sample a random number using the Ziggurat method (specifically the"] # [doc = " ZIGNOR variant from Doornik 2005). Most of the arguments are"] # [doc = " directly from the paper:"] # [doc = ""] # [doc = " * `rng`: source of randomness"] # [doc = " * `symmetric`: whether this is a symmetric distribution, or one-sided with P(x < 0) = 0."] # [doc = " * `X`: the $x_i$ abscissae."] # [doc = " * `F`: precomputed values of the PDF at the $x_i$, (i.e. $f(x_i)$)"] # [doc = " * `F_DIFF`: precomputed values of $f(x_i) - f(x_{i+1})$"] # [doc = " * `pdf`: the probability density function"] # [doc = " * `zero_case`: manual sampling from the tail when we chose the"] # [doc = "   bottom box (i.e. i == 0)"] # [inline (always)] pub (crate) fn ziggurat < R : Rng + ? Sized , P , Z > (rng : & mut R , symmetric : bool , x_tab : ziggurat_tables :: ZigTable , f_tab : ziggurat_tables :: ZigTable , mut pdf : P , mut zero_case : Z ,) -> f64 where P : FnMut (f64) -> f64 , Z : FnMut (& mut R , f64) -> f64 , { loop { let bits = rng . next_u64 () ; let i = bits as usize & 0xff ; let u = if symmetric { (bits >> 12) . into_float_with_exponent (1) - 3.0 } else { (bits >> 12) . into_float_with_exponent (0) - (1.0 - f64 :: EPSILON / 2.0) } ; let x = u * x_tab [i] ; let test_x = if symmetric { x . abs () } else { x } ; if test_x < x_tab [i + 1] { return x ; } if i == 0 { return zero_case (rng , u) ; } if f_tab [i + 1] + (f_tab [i] - f_tab [i + 1]) * rng . random :: < f64 > () < pdf (x) { return x ; } } }
};
}
