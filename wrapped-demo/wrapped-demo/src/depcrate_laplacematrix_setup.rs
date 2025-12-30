// Generated macro for matrix_setup (function)
macro_rules! Depcrate_laplacematrix_setup {
() => {
// Module: crate::laplace
// Provides: {"matrix_setup"}
// Dependencies: {}
fn matrix_setup (size_x : usize , size_y : usize) -> vec :: Vec < f64 > { let mut matrix = vec ! [0.0 ; size_x * size_y] ; for f in matrix . iter_mut () . take (size_x) { * f = 1.0 ; } for x in 0 .. size_x { matrix [(size_y - 1) * size_x + x] = 1.0 ; } for y in 0 .. size_y { matrix [y * size_x] = 1.0 ; } for y in 0 .. size_y { matrix [y * size_x + size_x - 1] = 1.0 ; } matrix }
};
}
