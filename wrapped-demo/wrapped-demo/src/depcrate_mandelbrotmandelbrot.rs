// Generated macro for mandelbrot (function)
macro_rules! Depcrate_mandelbrotmandelbrot {
() => {
// Module: crate::mandelbrot
// Provides: {"mandelbrot"}
// Dependencies: {}
pub fn mandelbrot () { eprintln ! () ; eprintln ! ("Mandelbrot Set ({WIDTH}x{HEIGHT})") ; let seq_count = calculate_mandelbrot (Mode :: Sequential) ; let par_count = calculate_mandelbrot (Mode :: Parallel) ; assert_eq ! (seq_count , par_count , "Sequential and parallel results differ!") ; }
};
}
