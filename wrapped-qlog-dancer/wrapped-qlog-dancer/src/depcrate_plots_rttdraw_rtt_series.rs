// Generated macro for draw_rtt_series (function)
macro_rules! Depcrate_plots_rttdraw_rtt_series {
() => {
// Module: crate::plots::rtt
// Provides: {"draw_rtt_series"}
// Dependencies: {}
fn draw_rtt_series < DB : DrawingBackend > (data : & [(f32 , f32)] , label : & str , colour : RGBColor , rtt_chart : & mut ChartContext < DB , Cartesian2d < RangedCoordf32 , RangedCoordf32 > > ,) { rtt_chart . draw_series (LineSeries :: new (data . to_vec () , colour)) . unwrap () . label (label) . legend (move | (x , y) | { PathElement :: new (vec ! [(x , y) , (x + 20 , y)] , colour) }) ; }
};
}
