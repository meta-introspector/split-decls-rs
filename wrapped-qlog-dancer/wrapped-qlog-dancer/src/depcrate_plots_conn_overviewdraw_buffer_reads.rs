// Generated macro for draw_buffer_reads (function)
macro_rules! Depcrate_plots_conn_overviewdraw_buffer_reads {
() => {
// Module: crate::plots::conn_overview
// Provides: {"draw_buffer_reads"}
// Dependencies: {}
fn draw_buffer_reads < DB : DrawingBackend > (streams : & BTreeMap < u64 , Vec < QlogPointu64 > > , stream_chart : & mut ChartContext < DB , Cartesian2d < RangedCoordf32 , RangedCoordu64 > , > ,) { let mut label = Some ("stream buffer read") ; for series in streams { draw_line (series . 1 , label , FOREST_GREEN , stream_chart) ; label = None ; } }
};
}
