// Generated macro for common (function)
macro_rules! Depcrate_math_j0common {
() => {
// Module: crate::math::j0
// Provides: {"common"}
// Dependencies: {}
fn common (ix : u32 , x : f64 , y0 : bool) -> f64 { let s : f64 ; let mut c : f64 ; let mut ss : f64 ; let mut cc : f64 ; let z : f64 ; s = sin (x) ; c = cos (x) ; if y0 { c = - c ; } cc = s + c ; if ix < 0x7fe00000 { ss = s - c ; z = - cos (2.0 * x) ; if s * c < 0.0 { cc = z / ss ; } else { ss = z / cc ; } if ix < 0x48000000 { if y0 { ss = - ss ; } cc = pzero (x) * cc - qzero (x) * ss ; } } return INVSQRTPI * cc / sqrt (x) ; }
};
}
