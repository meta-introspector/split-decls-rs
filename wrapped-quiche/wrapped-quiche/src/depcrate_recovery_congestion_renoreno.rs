// Generated macro for RENO (static)
macro_rules! Depcrate_recovery_congestion_renoRENO {
() => {
// Module: crate::recovery::congestion::reno
// Provides: {"RENO"}
// Dependencies: {}
pub (crate) static RENO : CongestionControlOps = CongestionControlOps { on_init , on_packet_sent , on_packets_acked , congestion_event , checkpoint , rollback , has_custom_pacing , # [cfg (feature = "qlog")] state_str , debug_fmt , } ;
};
}
