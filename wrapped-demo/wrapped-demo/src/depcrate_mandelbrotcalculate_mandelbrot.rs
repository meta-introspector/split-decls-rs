// Generated macro for calculate_mandelbrot (function)
macro_rules! Depcrate_mandelbrotcalculate_mandelbrot {
() => {
// Module: crate::mandelbrot
// Provides: {"calculate_mandelbrot"}
// Dependencies: {}
fn calculate_mandelbrot (mode : Mode) -> u64 { eprintln ! () ; eprint ! ("Calculating Mandelbrot {:10}" , format ! ("({mode:?}): ")) ; let scale_x = SCALE / WIDTH as f64 ; let scale_y = SCALE / HEIGHT as f64 ; let now = Instant :: now () ; let iter_count : u64 = match mode { Mode :: Sequential => { let mut sum = 0u64 ; for y in 0 .. HEIGHT { for x in 0 .. WIDTH { let cx = (x as f64) * scale_x - SCALE / 2.0 ; let cy = (y as f64) * scale_y - SCALE / 2.0 ; sum += mandelbrot_at_point (cx , cy) as u64 ; } } sum } Mode :: Parallel => (0 .. HEIGHT) . into_par_iter () . map (| y | { (0 .. WIDTH) . into_par_iter () . map (| x | { let cx = (x as f64) * scale_x - SCALE / 2.0 ; let cy = (y as f64) * scale_y - SCALE / 2.0 ; mandelbrot_at_point (cx , cy) as u64 }) . sum :: < u64 > () }) . sum () , } ; let elapsed = now . elapsed () ; eprintln ! ("{elapsed:?}") ; iter_count }
};
}
