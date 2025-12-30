// Generated macro for received_data (function)
macro_rules! Depcrate_plots_conn_flow_controlreceived_data {
() => {
// Module: crate::plots::conn_flow_control
// Provides: {"received_data"}
// Dependencies: {}
fn received_data < DB : DrawingBackend > (ss : & SeriesStore , proto : & ApplicationProto , chart : & mut ChartContext < '_ , DB , Cartesian2d < RangedCoordf32 , RangedCoordu64 > > ,) { let points = match proto { ApplicationProto :: Http2 => ss . netlog_h2_stream_received_connection_cumulative . clone () , ApplicationProto :: Http3 => ss . netlog_quic_stream_received_connection_cumulative . clone () , } ; chart . draw_series (LineSeries :: new (points , ORANGE)) . unwrap () . label ("cumulative received stream data") . legend (| (x , y) | PathElement :: new (vec ! [(x , y) , (x + 20 , y)] , ORANGE)) ; }
};
}
