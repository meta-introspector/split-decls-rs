// Generated macro for draw_smoothed_rtt (function)
macro_rules! Depcrate_plots_rttdraw_smoothed_rtt {
() => {
// Module: crate::plots::rtt
// Provides: {"draw_smoothed_rtt"}
// Dependencies: {}
fn draw_smoothed_rtt < DB : DrawingBackend > (data : & [(f32 , f32)] , rtt_chart : & mut ChartContext < DB , Cartesian2d < RangedCoordf32 , RangedCoordf32 > > ,) { draw_rtt_series (data , "Smoothed RTT" , BROWN , rtt_chart) ; }
};
}
