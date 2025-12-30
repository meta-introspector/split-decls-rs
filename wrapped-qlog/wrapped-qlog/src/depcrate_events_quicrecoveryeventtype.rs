// Generated macro for RecoveryEventType (enum)
macro_rules! Depcrate_events_quicRecoveryEventType {
() => {
// Module: crate::events::quic
// Provides: {"RecoveryEventType"}
// Dependencies: {}
# [derive (Serialize , Deserialize , Clone , Copy , PartialEq , Eq , Debug)] # [serde (rename_all = "snake_case")] pub enum RecoveryEventType { ParametersSet , MetricsUpdated , CongestionStateUpdated , LossTimerUpdated , PacketLost , MarkedForRetransmit , }
};
}
