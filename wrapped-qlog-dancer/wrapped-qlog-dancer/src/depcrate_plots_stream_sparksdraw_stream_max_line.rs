// Generated macro for draw_stream_max_line (function)
macro_rules! Depcrate_plots_stream_sparksdraw_stream_max_line {
() => {
// Module: crate::plots::stream_sparks
// Provides: {"draw_stream_max_line"}
// Dependencies: {}
fn draw_stream_max_line < DB : DrawingBackend > (stream_id : u64 , ss : & SeriesStore , vantage_point : & VantagePoint , transmission_type : TransmissionType , abs_chart : & mut ChartContext < DB , Cartesian2d < RangedCoordf32 , RangedCoordu64 > > , rel_chart : & mut ChartContext < DB , Cartesian2d < RangedCoordf32 , RangedCoordu64 > > ,) { let stream_max_data_to_plot = match (vantage_point , transmission_type) { (VantagePoint :: Client , TransmissionType :: Download) => ss . sent_stream_max_data . get (& stream_id) , (VantagePoint :: Client , TransmissionType :: Upload) => ss . received_stream_max_data . get (& stream_id) , (VantagePoint :: Server , TransmissionType :: Download) => ss . received_stream_max_data . get (& stream_id) , (VantagePoint :: Server , TransmissionType :: Upload) => ss . sent_stream_max_data . get (& stream_id) , } ; if let Some (stream_max_data) = stream_max_data_to_plot { abs_chart . draw_series (LineSeries :: new (stream_max_data . clone () , MUSTARD)) . unwrap () ; rel_chart . draw_series (LineSeries :: new (stream_max_data . clone () , MUSTARD)) . unwrap () ; } }
};
}
