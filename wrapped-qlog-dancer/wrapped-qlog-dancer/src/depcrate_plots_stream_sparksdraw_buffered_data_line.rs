// Generated macro for draw_buffered_data_line (function)
macro_rules! Depcrate_plots_stream_sparksdraw_buffered_data_line {
() => {
// Module: crate::plots::stream_sparks
// Provides: {"draw_buffered_data_line"}
// Dependencies: {}
fn draw_buffered_data_line < DB : DrawingBackend > (stream_id : u64 , ss : & SeriesStore , vantage_point : & VantagePoint , abs_chart : & mut ChartContext < DB , Cartesian2d < RangedCoordf32 , RangedCoordu64 > > , rel_chart : & mut ChartContext < DB , Cartesian2d < RangedCoordf32 , RangedCoordu64 > > ,) { let buffered_data_to_plot = match vantage_point { VantagePoint :: Client => ss . stream_buffer_reads . get (& stream_id) , VantagePoint :: Server => ss . stream_buffer_writes . get (& stream_id) , } ; if let Some (buffered_data) = buffered_data_to_plot { abs_chart . draw_series (LineSeries :: new (buffered_data . clone () , MAGENTA)) . unwrap () ; rel_chart . draw_series (LineSeries :: new (buffered_data . clone () , MAGENTA)) . unwrap () ; rel_chart . draw_series (buffered_data . iter () . map (| point | TriangleMarker :: new (* point , 3 , MAGENTA)) ,) . unwrap () ; } }
};
}
