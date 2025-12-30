// Generated macro for TransportError (enum)
macro_rules! Depcrate_events_quicTransportError {
() => {
// Module: crate::events::quic
// Provides: {"TransportError"}
// Dependencies: {}
# [derive (Serialize , Deserialize , Clone , PartialEq , Eq , Debug)] # [serde (rename_all = "snake_case")] pub enum TransportError { NoError , InternalError , ConnectionRefused , FlowControlError , StreamLimitError , StreamStateError , FinalSizeError , FrameEncodingError , TransportParameterError , ConnectionIdLimitError , ProtocolViolation , InvalidToken , ApplicationError , CryptoBufferExceeded , KeyUpdateError , AeadLimitReached , NoViablePath , }
};
}
