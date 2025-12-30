// Generated macro for map_error (function)
macro_rules! Depcrate_errormap_error {
() => {
// Module: crate::error
// Provides: {"map_error"}
// Dependencies: {}
pub (crate) fn map_error (input : Error) -> rustls_result { use rustls_result :: * ; match input { Error :: InappropriateMessage { .. } => InappropriateMessage , Error :: InappropriateHandshakeMessage { .. } => InappropriateHandshakeMessage , Error :: NoCertificatesPresented => NoCertificatesPresented , Error :: DecryptError => DecryptError , Error :: PeerIncompatible (_) => PeerIncompatibleError , Error :: PeerMisbehaved (_) => PeerMisbehavedError , Error :: UnsupportedNameType => UnsupportedNameType , Error :: EncryptError => EncryptError , Error :: InvalidMessage (e) => map_invalid_message_error (e) , Error :: FailedToGetCurrentTime => FailedToGetCurrentTime , Error :: FailedToGetRandomBytes => FailedToGetRandomBytes , Error :: HandshakeNotComplete => HandshakeNotComplete , Error :: PeerSentOversizedRecord => PeerSentOversizedRecord , Error :: NoApplicationProtocol => NoApplicationProtocol , Error :: BadMaxFragmentSize => BadMaxFragmentSize , Error :: InvalidCertificate (e) => map_invalid_certificate_error (e) , Error :: General (_) => General , Error :: AlertReceived (e) => map_alert_error (e) , Error :: InvalidCertRevocationList (e) => map_crl_error (e) , Error :: InconsistentKeys (InconsistentKeys :: KeyMismatch) => InconsistentKeysKeysMismatch , Error :: InconsistentKeys (InconsistentKeys :: Unknown) => InconsistentKeysUnknown , Error :: InvalidEncryptedClientHello (err) => map_ech_error (err) , _ => General , } }
};
}
