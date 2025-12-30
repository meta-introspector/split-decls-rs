// Generated macro for draw_sweep_line (function)
macro_rules! Depcrate_rasterizer_circledraw_sweep_line {
() => {
// Module: crate::rasterizer::circle
// Provides: {"draw_sweep_line"}
// Dependencies: {}
fn draw_sweep_line < B : DrawingBackend , S : BackendStyle > (b : & mut B , style : & S , (x0 , y0) : BackendCoord , (dx , dy) : (i32 , i32) , p0 : i32 , (s , e) : (f64 , f64) ,) -> Result < () , DrawingErrorKind < B :: ErrorType > > { let mut s = if dx < 0 || dy < 0 { - s } else { s } ; let mut e = if dx < 0 || dy < 0 { - e } else { e } ; if s > e { std :: mem :: swap (& mut s , & mut e) ; } let vs = s . ceil () - s ; let ve = e - e . floor () ; if dx == 0 { check_result ! (b . draw_line ((p0 + x0 , s . ceil () as i32 + y0) , (p0 + x0 , e . floor () as i32 + y0) , & style . color ())) ; check_result ! (b . draw_pixel ((p0 + x0 , s . ceil () as i32 + y0 - 1) , style . color () . mix (vs))) ; check_result ! (b . draw_pixel ((p0 + x0 , e . floor () as i32 + y0 + 1) , style . color () . mix (ve))) ; } else { check_result ! (b . draw_line ((s . ceil () as i32 + x0 , p0 + y0) , (e . floor () as i32 + x0 , p0 + y0) , & style . color ())) ; check_result ! (b . draw_pixel ((s . ceil () as i32 + x0 - 1 , p0 + y0) , style . color () . mix (vs))) ; check_result ! (b . draw_pixel ((e . floor () as i32 + x0 + 1 , p0 + y0) , style . color () . mix (ve))) ; } Ok (()) }
};
}
