// Generated macro for draw_sent_stream_data (function)
macro_rules! Depcrate_plots_conn_overviewdraw_sent_stream_data {
() => {
// Module: crate::plots::conn_overview
// Provides: {"draw_sent_stream_data"}
// Dependencies: {}
fn draw_sent_stream_data < DB : DrawingBackend > (streams : & BTreeMap < u64 , Vec < QlogPointu64 > > , stream_chart : & mut ChartContext < DB , Cartesian2d < RangedCoordf32 , RangedCoordu64 > , > ,) { let mut label = Some ("sent stream data") ; for series in streams { draw_line (series . 1 , label , PURPLE_500 , stream_chart) ; label = None ; } }
};
}
