// Generated macro for draw_bytes_in_flight (function)
macro_rules! Depcrate_plots_congestion_controldraw_bytes_in_flight {
() => {
// Module: crate::plots::congestion_control
// Provides: {"draw_bytes_in_flight"}
// Dependencies: {}
fn draw_bytes_in_flight < DB : DrawingBackend > (data : & [(f32 , u64)] , congestion_chart : & mut ChartContext < DB , Cartesian2d < RangedCoordf32 , RangedCoordu64 > , > ,) { draw_line (data , Some ("bytes in flight") , TAUPE , congestion_chart) ; }
};
}
