// Generated macro for blocked_lines (function)
macro_rules! Depcrate_plots_conn_flow_controlblocked_lines {
() => {
// Module: crate::plots::conn_flow_control
// Provides: {"blocked_lines"}
// Dependencies: {}
fn blocked_lines < DB : DrawingBackend > (ds : & Datastore , y_max : u64 , chart : & mut ChartContext < '_ , DB , Cartesian2d < RangedCoordf32 , RangedCoordu64 > > ,) { if let Some (blocked) = ds . netlog_quic_server_window_blocked . get (& - 1) . cloned () { let blocked_lines = blocked . iter () . map (| t | PathElement :: new ([(* t , 0) , (* t , y_max)] , GREEN)) ; chart . draw_series (blocked_lines) . unwrap () . label ("server conn blocked") . legend (| (x , y) | PathElement :: new (vec ! [(x , y) , (x + 20 , y)] , GREEN)) ; } }
};
}
