// Generated macro for draw_part_a (function)
macro_rules! Depcrate_rasterizer_circledraw_part_a {
() => {
// Module: crate::rasterizer::circle
// Provides: {"draw_part_a"}
// Dependencies: {}
fn draw_part_a < B : DrawingBackend , Draw : FnMut (i32 , (f64 , f64)) -> Result < () , DrawingErrorKind < B :: ErrorType > > , > (height : f64 , radius : u32 , mut draw : Draw ,) -> Result < () , DrawingErrorKind < B :: ErrorType > > { let half_width = (radius as f64 * radius as f64 - (radius as f64 - height) * (radius as f64 - height)) . sqrt () ; let x0 = (- half_width) . ceil () as i32 ; let x1 = half_width . floor () as i32 ; let y0 = (radius as f64 - height) . ceil () ; for x in x0 ..= x1 { let y1 = (radius as f64 * radius as f64 - x as f64 * x as f64) . sqrt () ; check_result ! (draw (x , (y0 , y1))) ; } Ok (()) }
};
}
