// Generated macro for rodrigues_rotate_point (function)
macro_rules! Depcrate_saferodrigues_rotate_point {
() => {
// Module: crate::safe
// Provides: {"rodrigues_rotate_point"}
// Dependencies: {}
fn rodrigues_rotate_point (rot : & [f64 ; 3] , pt : & [f64 ; 3] , rotated_pt : & mut [f64 ; 3]) { let sqtheta = sqsum (rot) ; if sqtheta != 0. { let theta = sqtheta . sqrt () ; let costheta = theta . cos () ; let sintheta = theta . sin () ; let theta_inverse = 1. / theta ; let mut w = [0. ; 3] ; for i in 0 .. 3 { w [i] = rot [i] * theta_inverse ; } let w_cross_pt = cross (& w , & pt) ; let tmp = (w [0] * pt [0] + w [1] * pt [1] + w [2] * pt [2]) * (1. - costheta) ; for i in 0 .. 3 { rotated_pt [i] = pt [i] * costheta + w_cross_pt [i] * sintheta + w [i] * tmp ; } } else { let rot_cross_pt = cross (& rot , & pt) ; for i in 0 .. 3 { rotated_pt [i] = pt [i] + rot_cross_pt [i] ; } } }
};
}
