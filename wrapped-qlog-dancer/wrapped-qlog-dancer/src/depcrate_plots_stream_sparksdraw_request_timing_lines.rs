// Generated macro for draw_request_timing_lines (function)
macro_rules! Depcrate_plots_stream_sparksdraw_request_timing_lines {
() => {
// Module: crate::plots::stream_sparks
// Provides: {"draw_request_timing_lines"}
// Dependencies: {}
fn draw_request_timing_lines < DB : DrawingBackend > (ds : & Datastore , stream_id : u64 , y_max : u64 , abs_chart : & mut ChartContext < DB , Cartesian2d < RangedCoordf32 , RangedCoordu64 > > , rel_chart : & mut ChartContext < DB , Cartesian2d < RangedCoordf32 , RangedCoordu64 > > ,) { if let Some (req) = ds . http_requests . get (& stream_id) { if let Some (h) = req . time_first_headers_rx { let points = vec ! [(h , 0) , (h , y_max)] ; abs_chart . draw_series (LineSeries :: new (points , GREEN)) . unwrap () ; } if let Some (h) = req . time_first_headers_tx { let points = vec ! [(h , 0) , (h , y_max)] ; abs_chart . draw_series (LineSeries :: new (points . clone () , CYAN)) . unwrap () ; rel_chart . draw_series (LineSeries :: new (points , CYAN)) . unwrap () ; } let (first_data_to_plot , last_data_to_plot) = match ds . vantage_point { VantagePoint :: Client => (req . time_first_data_rx , req . time_last_data_rx) , VantagePoint :: Server => (req . time_first_data_tx , req . time_last_data_tx) , } ; if let Some (t) = first_data_to_plot { let points = vec ! [(t , 0) , (t , y_max)] ; abs_chart . draw_series (LineSeries :: new (points . clone () , ORANGE)) . unwrap () ; rel_chart . draw_series (LineSeries :: new (points , ORANGE)) . unwrap () ; } if let Some (t) = last_data_to_plot { let points = vec ! [(t , 0) , (t , y_max)] ; abs_chart . draw_series (LineSeries :: new (points . clone () , BROWN)) . unwrap () ; rel_chart . draw_series (LineSeries :: new (points , BROWN)) . unwrap () ; } } }
};
}
