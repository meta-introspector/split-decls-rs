// Generated macro for map_invalid_message_error (function)
macro_rules! Depcrate_errormap_invalid_message_error {
() => {
// Module: crate::error
// Provides: {"map_invalid_message_error"}
// Dependencies: {}
fn map_invalid_message_error (err : InvalidMessage) -> rustls_result { use rustls_result :: * ; match err { InvalidMessage :: HandshakePayloadTooLarge => MessageHandshakePayloadTooLarge , InvalidMessage :: CertificatePayloadTooLarge => MessageCertificatePayloadTooLarge , InvalidMessage :: InvalidCcs => MessageInvalidCcs , InvalidMessage :: InvalidContentType => MessageInvalidContentType , InvalidMessage :: InvalidCertificateStatusType => MessageInvalidCertStatusType , InvalidMessage :: InvalidCertRequest => MessageInvalidCertRequest , InvalidMessage :: InvalidDhParams => MessageInvalidDhParams , InvalidMessage :: InvalidEmptyPayload => MessageInvalidEmptyPayload , InvalidMessage :: InvalidKeyUpdate => MessageInvalidKeyUpdate , InvalidMessage :: InvalidServerName => MessageInvalidServerName , InvalidMessage :: MessageTooLarge => MessageTooLarge , InvalidMessage :: MessageTooShort => MessageTooShort , InvalidMessage :: MissingData (_) => MessageMissingData , InvalidMessage :: MissingKeyExchange => MessageMissingKeyExchange , InvalidMessage :: NoSignatureSchemes => MessageNoSignatureSchemes , InvalidMessage :: TrailingData (_) => MessageTrailingData , InvalidMessage :: UnexpectedMessage (_) => MessageUnexpectedMessage , InvalidMessage :: UnknownProtocolVersion => MessageUnknownProtocolVersion , InvalidMessage :: UnsupportedCompression => MessageUnsupportedCompression , InvalidMessage :: UnsupportedCurveType => MessageUnsupportedCurveType , InvalidMessage :: UnsupportedKeyExchangeAlgorithm (_) => MessageUnsupportedCompression , _ => MessageInvalidOther , } }
};
}
