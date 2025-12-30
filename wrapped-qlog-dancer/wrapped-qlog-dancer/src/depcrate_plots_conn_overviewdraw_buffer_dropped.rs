// Generated macro for draw_buffer_dropped (function)
macro_rules! Depcrate_plots_conn_overviewdraw_buffer_dropped {
() => {
// Module: crate::plots::conn_overview
// Provides: {"draw_buffer_dropped"}
// Dependencies: {}
fn draw_buffer_dropped < DB : DrawingBackend > (streams : & BTreeMap < u64 , Vec < QlogPointu64 > > , stream_chart : & mut ChartContext < DB , Cartesian2d < RangedCoordf32 , RangedCoordu64 > , > ,) { let mut label = Some ("stream buffer dropped") ; for series in streams { draw_line (series . 1 , label , ORANGE , stream_chart) ; label = None ; } }
};
}
