// Generated macro for BBR (static)
macro_rules! Depcrate_recovery_congestion_bbrBBR {
() => {
// Module: crate::recovery::congestion::bbr
// Provides: {"BBR"}
// Dependencies: {}
pub (crate) static BBR : CongestionControlOps = CongestionControlOps { on_init , on_packet_sent , on_packets_acked , congestion_event , checkpoint , rollback , has_custom_pacing , # [cfg (feature = "qlog")] state_str , debug_fmt , } ;
};
}
