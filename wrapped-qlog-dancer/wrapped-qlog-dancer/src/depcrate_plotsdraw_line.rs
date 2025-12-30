// Generated macro for draw_line (function)
macro_rules! Depcrate_plotsdraw_line {
() => {
// Module: crate::plots
// Provides: {"draw_line"}
// Dependencies: {}
fn draw_line < DB : DrawingBackend > (data : & [(f32 , u64)] , label : Option < & str > , colour : RGBColor , chart : & mut ChartContext < DB , Cartesian2d < RangedCoordf32 , RangedCoordu64 > > ,) { let c = chart . draw_series (LineSeries :: new (data . to_vec () , colour)) . unwrap () ; if let Some (l) = label { c . label (l) . legend (move | (x , y) | { PathElement :: new (vec ! [(x , y) , (x + 20 , y)] , colour) }) ; } }
};
}
