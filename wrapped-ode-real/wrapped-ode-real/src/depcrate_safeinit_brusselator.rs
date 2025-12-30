// Generated macro for init_brusselator (function)
macro_rules! Depcrate_safeinit_brusselator {
() => {
// Module: crate::safe
// Provides: {"init_brusselator"}
// Dependencies: {}
# [expect (unused)] fn init_brusselator (u : & mut [f64] , v : & mut [f64]) { assert ! (u . len () == N * N) ; assert ! (v . len () == N * N) ; for i in 0 .. N { for j in 0 .. N { let x = range (xmin , xmax , i , N) ; let y = range (ymin , ymax , j , N) ; u [N * i + j] = 22.0 * (y * (1.0 - y)) * (y * (1.0 - y)) . sqrt () ; v [N * i + j] = 27.0 * (x * (1.0 - x)) * (x * (1.0 - x)) . sqrt () ; } } }
};
}
