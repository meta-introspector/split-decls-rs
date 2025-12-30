// Generated macro for draw_received_max_data (function)
macro_rules! Depcrate_plots_conn_overviewdraw_received_max_data {
() => {
// Module: crate::plots::conn_overview
// Provides: {"draw_received_max_data"}
// Dependencies: {}
fn draw_received_max_data < DB : DrawingBackend > (data : & [(f32 , u64)] , stream_chart : & mut ChartContext < DB , Cartesian2d < RangedCoordf32 , RangedCoordu64 > , > ,) { draw_line (data , Some ("cumulative received MAX_DATA") , MID_GREY , stream_chart ,) ; }
};
}
