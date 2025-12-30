// Generated macro for project (function)
macro_rules! Depcrate_safeproject {
() => {
// Module: crate::safe
// Provides: {"project"}
// Dependencies: {}
fn project (cam : & [f64 ; 11] , X : & [f64 ; 3] , proj : & mut [f64 ; 2]) { let C = & cam [3 .. 6] ; let mut Xo = [0. ; 3] ; let mut Xcam = [0. ; 3] ; Xo [0] = X [0] - C [0] ; Xo [1] = X [1] - C [1] ; Xo [2] = X [2] - C [2] ; rodrigues_rotate_point (cam . first_chunk :: < 3 > () . unwrap () , & Xo , & mut Xcam) ; proj [0] = Xcam [0] / Xcam [2] ; proj [1] = Xcam [1] / Xcam [2] ; radial_distort (& cam [9 ..] , proj) ; proj [0] = proj [0] * cam [6] + cam [7] ; proj [1] = proj [1] * cam [6] + cam [8] ; }
};
}
