// Generated macro for window_updates (function)
macro_rules! Depcrate_plots_conn_flow_controlwindow_updates {
() => {
// Module: crate::plots::conn_flow_control
// Provides: {"window_updates"}
// Dependencies: {}
fn window_updates < DB : DrawingBackend > (ss : & SeriesStore , ds : & Datastore , chart : & mut ChartContext < '_ , DB , Cartesian2d < RangedCoordf32 , RangedCoordu64 > > ,) { let points = match ds . application_proto { ApplicationProto :: Http2 => { if let Some (conn_win_updates) = ss . h2_send_window_series_absolute . get (& 0) . cloned () { conn_win_updates } else { return ; } } , ApplicationProto :: Http3 => { if let Some (conn_win_updates) = ss . netlog_quic_client_side_window_updates . get (& - 1) . cloned () { conn_win_updates } else { return ; } } , } ; let circles : Vec < Circle < (f32 , u64) , i32 > > = points . iter () . map (| point | Circle :: new (* point , 2 , BLUE)) . collect () ; chart . draw_series (LineSeries :: new (points , BLUE)) . unwrap () . label ("sent window updates") . legend (| (x , y) | PathElement :: new (vec ! [(x , y) , (x + 20 , y)] , BLUE)) ; chart . draw_series (circles) . unwrap () ; }
};
}
