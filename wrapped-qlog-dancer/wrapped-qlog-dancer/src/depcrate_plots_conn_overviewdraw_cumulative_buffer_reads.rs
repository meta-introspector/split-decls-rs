// Generated macro for draw_cumulative_buffer_reads (function)
macro_rules! Depcrate_plots_conn_overviewdraw_cumulative_buffer_reads {
() => {
// Module: crate::plots::conn_overview
// Provides: {"draw_cumulative_buffer_reads"}
// Dependencies: {}
fn draw_cumulative_buffer_reads < DB : DrawingBackend > (data : & [(f32 , u64)] , stream_chart : & mut ChartContext < DB , Cartesian2d < RangedCoordf32 , RangedCoordu64 > , > ,) { draw_line (data , Some ("cumulative stream buffer read") , GREEN , stream_chart ,) ; }
};
}
