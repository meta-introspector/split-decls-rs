// Generated macro for cc_state_to_color (function)
macro_rules! Depcrate_plots_congestion_controlcc_state_to_color {
() => {
// Module: crate::plots::congestion_control
// Provides: {"cc_state_to_color"}
// Dependencies: {}
fn cc_state_to_color (cc_state : & str) -> RGBColor { match cc_state { "slow_start" => RGBColor (204 , 81 , 81) , "recovery" => RGBColor (127 , 51 , 51) , "congestion_avoidance" => RGBColor (81 , 204 , 204) , "bbr_startup" => RGBColor (204 , 81 , 81) , "bbr_drain" => RGBColor (127 , 51 , 51) , "bbr_probe_bw" => RGBColor (81 , 204 , 204) , "bbr_probe_rtt" => RGBColor (51 , 127 , 127) , _ => BLACK , } }
};
}
