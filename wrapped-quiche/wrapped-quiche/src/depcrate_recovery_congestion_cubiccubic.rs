// Generated macro for CUBIC (static)
macro_rules! Depcrate_recovery_congestion_cubicCUBIC {
() => {
// Module: crate::recovery::congestion::cubic
// Provides: {"CUBIC"}
// Dependencies: {}
pub (crate) static CUBIC : CongestionControlOps = CongestionControlOps { on_init , on_packet_sent , on_packets_acked , congestion_event , checkpoint , rollback , has_custom_pacing , # [cfg (feature = "qlog")] state_str , debug_fmt , } ;
};
}
