// Generated macro for brusselator_2d_loop_unsf (function)
macro_rules! Depcrate_unsfbrusselator_2d_loop_unsf {
() => {
// Module: crate::unsf
// Provides: {"brusselator_2d_loop_unsf"}
// Dependencies: {}
# [no_mangle] # [autodiff (dbrusselator_2d_loop_unsf , Reverse , Duplicated , Duplicated , Duplicated , Duplicated , Duplicated , Const)] pub unsafe fn brusselator_2d_loop_unsf (d_u : * mut f64 , d_v : * mut f64 , u : * const f64 , v : * const f64 , p : * const f64 , t : f64) { let A = * p . add (0) ; let B = * p . add (1) ; let alpha = * p . add (2) ; let dx = 1. / (N - 1) as f64 ; let alpha = alpha / (dx * dx) ; for i in 0 .. N { for j in 0 .. N { let x = range (xmin , xmax , i , N) ; let y = range (ymin , ymax , j , N) ; let ip1 = if i == N - 1 { i } else { i + 1 } ; let im1 = if i == 0 { i } else { i - 1 } ; let jp1 = if j == N - 1 { j } else { j + 1 } ; let jm1 = if j == 0 { j } else { j - 1 } ; let u2v = * u . add (N * i + j) * * u . add (N * i + j) * * v . add (N * i + j) ; * d_u . add (N * i + j) = alpha * (* u . add (N * im1 + j) + * u . add (N * ip1 + j) + * u . add (N * i + jp1) + * u . add (N * i + jm1) - 4. * * u . add (N * i + j)) + B + u2v - (A + 1.) * * u . add (N * i + j) + brusselator_f (x , y , t) ; * d_v . add (N * i + j) = alpha * (* v . add (N * im1 + j) + * v . add (N * ip1 + j) + * v . add (N * i + jp1) + * v . add (N * i + jm1) - 4. * * v . add (N * i + j)) + A * * u . add (N * i + j) - u2v ; } } }
};
}
