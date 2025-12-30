// Generated macro for common (function)
macro_rules! Depcrate_math_j0fcommon {
() => {
// Module: crate::math::j0f
// Provides: {"common"}
// Dependencies: {}
fn common (ix : u32 , x : f32 , y0 : bool) -> f32 { let z : f32 ; let s : f32 ; let mut c : f32 ; let mut ss : f32 ; let mut cc : f32 ; s = sinf (x) ; c = cosf (x) ; if y0 { c = - c ; } cc = s + c ; if ix < 0x7f000000 { ss = s - c ; z = - cosf (2.0 * x) ; if s * c < 0.0 { cc = z / ss ; } else { ss = z / cc ; } if ix < 0x58800000 { if y0 { ss = - ss ; } cc = pzerof (x) * cc - qzerof (x) * ss ; } } return INVSQRTPI * cc / sqrtf (x) ; }
};
}
