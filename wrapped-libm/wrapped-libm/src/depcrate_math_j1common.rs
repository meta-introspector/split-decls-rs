// Generated macro for common (function)
macro_rules! Depcrate_math_j1common {
() => {
// Module: crate::math::j1
// Provides: {"common"}
// Dependencies: {}
fn common (ix : u32 , x : f64 , y1 : bool , sign : bool) -> f64 { let z : f64 ; let mut s : f64 ; let c : f64 ; let mut ss : f64 ; let mut cc : f64 ; s = sin (x) ; if y1 { s = - s ; } c = cos (x) ; cc = s - c ; if ix < 0x7fe00000 { ss = - s - c ; z = cos (2.0 * x) ; if s * c > 0.0 { cc = z / ss ; } else { ss = z / cc ; } if ix < 0x48000000 { if y1 { ss = - ss ; } cc = pone (x) * cc - qone (x) * ss ; } } if sign { cc = - cc ; } return INVSQRTPI * cc / sqrt (x) ; }
};
}
