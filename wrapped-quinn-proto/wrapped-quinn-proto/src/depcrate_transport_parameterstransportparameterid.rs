// Generated macro for TransportParameterId (enum)
macro_rules! Depcrate_transport_parametersTransportParameterId {
() => {
// Module: crate::transport_parameters
// Provides: {"TransportParameterId"}
// Dependencies: {}
# [repr (u64)] # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub (crate) enum TransportParameterId { OriginalDestinationConnectionId = 0x00 , MaxIdleTimeout = 0x01 , StatelessResetToken = 0x02 , MaxUdpPayloadSize = 0x03 , InitialMaxData = 0x04 , InitialMaxStreamDataBidiLocal = 0x05 , InitialMaxStreamDataBidiRemote = 0x06 , InitialMaxStreamDataUni = 0x07 , InitialMaxStreamsBidi = 0x08 , InitialMaxStreamsUni = 0x09 , AckDelayExponent = 0x0A , MaxAckDelay = 0x0B , DisableActiveMigration = 0x0C , PreferredAddress = 0x0D , ActiveConnectionIdLimit = 0x0E , InitialSourceConnectionId = 0x0F , RetrySourceConnectionId = 0x10 , ReservedTransportParameter = 0x1B , MaxDatagramFrameSize = 0x20 , GreaseQuicBit = 0x2AB2 , MinAckDelayDraft07 = 0xFF04DE1B , }
};
}
