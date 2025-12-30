// Generated macro for laplace (function)
macro_rules! Depcrate_laplacelaplace {
() => {
// Module: crate::laplace
// Provides: {"laplace"}
// Dependencies: {}
pub fn laplace () { eprintln ! () ; let mut matrix = matrix_setup (SIZE , SIZE) ; eprintln ! ("Laplace iterations") ; let now = Instant :: now () ; let residual = compute (& mut matrix , SIZE , SIZE , ITERATIONS) ; let elapsed = now . elapsed () ; eprintln ! ("{ITERATIONS} iterations: {elapsed:?} (residual: {residual})") ; assert ! (residual < 0.001) ; }
};
}
