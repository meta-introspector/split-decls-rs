// Generated macro for draw_part_c (function)
macro_rules! Depcrate_rasterizer_circledraw_part_c {
() => {
// Module: crate::rasterizer::circle
// Provides: {"draw_part_c"}
// Dependencies: {}
fn draw_part_c < B : DrawingBackend , Draw : FnMut (i32 , (f64 , f64)) -> Result < () , DrawingErrorKind < B :: ErrorType > > , > (r : i32 , r_limit : i32 , mut draw : Draw ,) -> Result < () , DrawingErrorKind < B :: ErrorType > > { let half_size = r as f64 / (2f64) . sqrt () ; let (x0 , x1) = ((- half_size) . ceil () as i32 , half_size . floor () as i32) ; for x in x0 .. x1 { let outer_y0 = ((r_limit as f64) * (r_limit as f64) - x as f64 * x as f64) . sqrt () ; let inner_y0 = r as f64 - 1.0 ; let mut y1 = outer_y0 . min (inner_y0) ; let y0 = ((r as f64) * (r as f64) - x as f64 * x as f64) . sqrt () ; if y0 > y1 { y1 = y0 . ceil () ; if y1 >= r as f64 { continue ; } } check_result ! (draw (x , (y0 , y1))) ; } for x in x1 + 1 .. r { let outer_y0 = ((r_limit as f64) * (r_limit as f64) - x as f64 * x as f64) . sqrt () ; let inner_y0 = r as f64 - 1.0 ; let y0 = outer_y0 . min (inner_y0) ; let y1 = x as f64 ; if y1 < y0 { check_result ! (draw (x , (y0 , y1 + 1.0))) ; check_result ! (draw (- x , (y0 , y1 + 1.0))) ; } } Ok (()) }
};
}
