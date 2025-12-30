// Generated macro for brusselator_2d_loop (function)
macro_rules! Depcrate_safebrusselator_2d_loop {
() => {
// Module: crate::safe
// Provides: {"brusselator_2d_loop"}
// Dependencies: {}
# [no_mangle] # [autodiff (dbrusselator_2d_loop , Reverse , Duplicated , Duplicated , Duplicated , Duplicated , Duplicated , Const)] pub fn brusselator_2d_loop (d_u : & mut [f64 ; N * N] , d_v : & mut [f64 ; N * N] , u : & [f64 ; N * N] , v : & [f64 ; N * N] , p : & [f64 ; 3] , t : f64) { let A = p [0] ; let B = p [1] ; let alpha = p [2] ; let dx = 1. / (N - 1) as f64 ; let alpha = alpha / (dx * dx) ; for i in 0 .. N { for j in 0 .. N { let x = range (xmin , xmax , i , N) ; let y = range (ymin , ymax , j , N) ; let ip1 = if i == N - 1 { i } else { i + 1 } ; let im1 = if i == 0 { i } else { i - 1 } ; let jp1 = if j == N - 1 { j } else { j + 1 } ; let jm1 = if j == 0 { j } else { j - 1 } ; let u2v = u [N * i + j] * u [N * i + j] * v [N * i + j] ; d_u [N * i + j] = alpha * (u [N * im1 + j] + u [N * ip1 + j] + u [N * i + jp1] + u [N * i + jm1] - 4. * u [N * i + j]) + B + u2v - (A + 1.) * u [N * i + j] + brusselator_f (x , y , t) ; d_v [N * i + j] = alpha * (v [N * im1 + j] + v [N * ip1 + j] + v [N * i + jp1] + v [N * i + jm1] - 4. * v [N * i + j]) + A * u [N * i + j] - u2v ; } } }
};
}
