// Generated macro for draw_latest_rtt (function)
macro_rules! Depcrate_plots_rttdraw_latest_rtt {
() => {
// Module: crate::plots::rtt
// Provides: {"draw_latest_rtt"}
// Dependencies: {}
fn draw_latest_rtt < DB : DrawingBackend > (data : & [(f32 , f32)] , rtt_chart : & mut ChartContext < DB , Cartesian2d < RangedCoordf32 , RangedCoordf32 > > ,) { draw_rtt_series (data , "Latest RTT" , ORANGE , rtt_chart) ; }
};
}
