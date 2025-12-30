// Generated macro for draw_part_b (function)
macro_rules! Depcrate_rasterizer_circledraw_part_b {
() => {
// Module: crate::rasterizer::circle
// Provides: {"draw_part_b"}
// Dependencies: {}
fn draw_part_b < B : DrawingBackend , Draw : FnMut (i32 , (f64 , f64)) -> Result < () , DrawingErrorKind < B :: ErrorType > > , > (from : f64 , size : f64 , mut draw : Draw ,) -> Result < () , DrawingErrorKind < B :: ErrorType > > { let from = from . floor () ; for x in (from - size) . floor () as i32 ..= from as i32 { check_result ! (draw (x , (- x as f64 , x as f64))) ; } Ok (()) }
};
}
