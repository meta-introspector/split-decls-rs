// Generated macro for draw_ssthresh (function)
macro_rules! Depcrate_plots_congestion_controldraw_ssthresh {
() => {
// Module: crate::plots::congestion_control
// Provides: {"draw_ssthresh"}
// Dependencies: {}
fn draw_ssthresh < DB : DrawingBackend > (data : & [(f32 , u64)] , congestion_chart : & mut ChartContext < DB , Cartesian2d < RangedCoordf32 , RangedCoordu64 > , > ,) { draw_line (data , Some ("ssthresh") , ORANGE , congestion_chart) ; }
};
}
