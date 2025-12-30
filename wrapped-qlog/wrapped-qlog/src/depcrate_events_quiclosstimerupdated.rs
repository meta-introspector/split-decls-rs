// Generated macro for LossTimerUpdated (struct)
macro_rules! Depcrate_events_quicLossTimerUpdated {
() => {
// Module: crate::events::quic
// Provides: {"LossTimerUpdated"}
// Dependencies: {}
# [serde_with :: skip_serializing_none] # [derive (Serialize , Deserialize , Clone , PartialEq , Debug)] pub struct LossTimerUpdated { pub timer_type : Option < TimerType > , pub packet_number_space : Option < PacketNumberSpace > , pub event_type : LossTimerEventType , pub delta : Option < f32 > , }
};
}
