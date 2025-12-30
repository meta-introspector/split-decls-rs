// Generated macro for draw_min_rtt (function)
macro_rules! Depcrate_plots_rttdraw_min_rtt {
() => {
// Module: crate::plots::rtt
// Provides: {"draw_min_rtt"}
// Dependencies: {}
fn draw_min_rtt < DB : DrawingBackend > (data : & [(f32 , f32)] , rtt_chart : & mut ChartContext < DB , Cartesian2d < RangedCoordf32 , RangedCoordf32 > > ,) { draw_rtt_series (data , "Min RTT" , SOFT_PINK , rtt_chart) ; }
};
}
