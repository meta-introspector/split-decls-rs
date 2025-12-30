// Generated macro for draw_sent_max_data (function)
macro_rules! Depcrate_plots_conn_overviewdraw_sent_max_data {
() => {
// Module: crate::plots::conn_overview
// Provides: {"draw_sent_max_data"}
// Dependencies: {}
fn draw_sent_max_data < DB : DrawingBackend > (data : & [(f32 , u64)] , stream_chart : & mut ChartContext < DB , Cartesian2d < RangedCoordf32 , RangedCoordu64 > , > ,) { draw_line (data , Some ("sent MAX_DATA") , BLACK , stream_chart) ; }
};
}
