// Generated macro for BBR2 (static)
macro_rules! Depcrate_recovery_congestion_bbr2BBR2 {
() => {
// Module: crate::recovery::congestion::bbr2
// Provides: {"BBR2"}
// Dependencies: {}
pub (crate) static BBR2 : CongestionControlOps = CongestionControlOps { on_init , on_packet_sent , on_packets_acked , congestion_event , checkpoint , rollback , has_custom_pacing , # [cfg (feature = "qlog")] state_str , debug_fmt , } ;
};
}
