// Generated macro for draw_cwnd (function)
macro_rules! Depcrate_plots_congestion_controldraw_cwnd {
() => {
// Module: crate::plots::congestion_control
// Provides: {"draw_cwnd"}
// Dependencies: {}
fn draw_cwnd < DB : DrawingBackend > (data : & [(f32 , u64)] , congestion_chart : & mut ChartContext < DB , Cartesian2d < RangedCoordf32 , RangedCoordu64 > , > ,) { draw_line (data , Some ("cwnd") , PURPLE_500 , congestion_chart) ; }
};
}
