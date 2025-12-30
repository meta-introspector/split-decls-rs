// Generated macro for draw_cumulative_sent_max_data (function)
macro_rules! Depcrate_plots_conn_overviewdraw_cumulative_sent_max_data {
() => {
// Module: crate::plots::conn_overview
// Provides: {"draw_cumulative_sent_max_data"}
// Dependencies: {}
fn draw_cumulative_sent_max_data < DB : DrawingBackend > (data : & [(f32 , u64)] , stream_chart : & mut ChartContext < DB , Cartesian2d < RangedCoordf32 , RangedCoordu64 > , > ,) { draw_line (data , Some ("cumulative sent MAX_STREAM_DATA") , CYAN , stream_chart ,) ; }
};
}
