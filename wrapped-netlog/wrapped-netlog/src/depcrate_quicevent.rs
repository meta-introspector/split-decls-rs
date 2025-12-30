// Generated macro for Event (enum)
macro_rules! Depcrate_quicEvent {
() => {
// Module: crate::quic
// Provides: {"Event"}
// Dependencies: {}
# [derive (Debug)] pub enum Event { QuicSession (QuicSessionEvent) , QuicSessionTransportParametersSent (QuicSessionTransportParametersSentEvent) , QuicSessionTransportParametersReceived (QuicSessionTransportParametersReceivedEvent ,) , QuicSessionUnauthenticatedPacketHeaderReceived (QuicSessionUnauthenticatedPacketHeaderReceived ,) , QuicSessionPacketSent (QuicSessionPacketSent) , QuicSessionAckFrameSent (QuicSessionAckFrameSent) , QuicSessionAckFrameReceived (QuicSessionAckFrameReceived) , QuicSessionStreamFrameReceived (QuicSessionStreamFrameReceivedEvent) , QuicSessionStopSendingFrameSent (QuicSessionStopSendingFrameSentEvent) , QuicSessionRstStreamFrameSent (QuicSessionRstStreamFrameSentEvent) , QuicSessionRstStreamFrameReceived (QuicSessionRstStreamFrameReceivedEvent) , QuicSessionBlockedFrameReceived (QuicSessionBlockedFrameReceivedEvent) , QuicSessionWindowUpdateFrameSent (QuicSessionWindowUpdateFrameSentEvent) , QuicSessionClosed (QuicSessionClosedEvent) , }
};
}
