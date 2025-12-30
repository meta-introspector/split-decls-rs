// Generated macro for impl_197 (impl)
macro_rules! Depcrate_plots_conn_flow_controlimpl_197 {
() => {
// Module: crate::plots::conn_flow_control
// Provides: {"impl_197"}
// Dependencies: {}
impl XYMinMax { fn init (params : & PlotParameters , ss : & SeriesStore , ds : & Datastore) -> Self { let x = XMinMax :: new (ss . received_x_min , ss . received_x_max , params . clamp . start , params . clamp . end ,) ; Self { x , y_min : 0 , y_max : Self :: y_max (ss , & ds . application_proto) , } } fn y_range (& self) -> std :: ops :: Range < u64 > { self . y_min .. self . y_max } fn y_max (ss : & SeriesStore , proto : & ApplicationProto) -> u64 { let y = match proto { ApplicationProto :: Http2 => * (ss . h2_send_window_absolute_max . get (& 0) . unwrap_or (& 0)) , ApplicationProto :: Http3 => ss . netlog_quic_stream_received_connection_cumulative . last () . unwrap_or (& (0.0 , 0)) . 1 , } ; (y as f64 * Y_WIGGLE) as u64 } }
};
}
