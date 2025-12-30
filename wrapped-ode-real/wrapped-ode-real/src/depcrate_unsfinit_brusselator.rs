// Generated macro for init_brusselator (function)
macro_rules! Depcrate_unsfinit_brusselator {
() => {
// Module: crate::unsf
// Provides: {"init_brusselator"}
// Dependencies: {}
# [expect (unused)] unsafe fn init_brusselator (u : * mut f64 , v : * mut f64) { for i in 0 .. N { for j in 0 .. N { let x = range (xmin , xmax , i , N) ; let y = range (ymin , ymax , j , N) ; * u . add (N * i + j) = 22.0 * (y * (1.0 - y)) * (y * (1.0 - y)) . sqrt () ; * v . add (N * i + j) = 27.0 * (x * (1.0 - x)) * (x * (1.0 - x)) . sqrt () ; } } }
};
}
