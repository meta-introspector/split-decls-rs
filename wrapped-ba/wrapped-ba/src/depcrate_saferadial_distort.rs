// Generated macro for radial_distort (function)
macro_rules! Depcrate_saferadial_distort {
() => {
// Module: crate::safe
// Provides: {"radial_distort"}
// Dependencies: {}
fn radial_distort (rad_params : & [f64] , proj : & mut [f64]) { let rsq = sqsum (proj) ; let l = 1. + rad_params [0] * rsq + rad_params [1] * rsq * rsq ; proj [0] = proj [0] * l ; proj [1] = proj [1] * l ; }
};
}
