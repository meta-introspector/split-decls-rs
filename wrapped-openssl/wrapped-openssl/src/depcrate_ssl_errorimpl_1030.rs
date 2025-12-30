// Generated macro for impl_1030 (impl)
macro_rules! Depcrate_ssl_errorimpl_1030 {
() => {
// Module: crate::ssl::error
// Provides: {"impl_1030"}
// Dependencies: {}
impl < S : fmt :: Debug > fmt :: Display for HandshakeError < S > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { HandshakeError :: SetupFailure (ref e) => write ! (f , "stream setup failed: {}" , e) ? , HandshakeError :: Failure (ref s) => { write ! (f , "the handshake failed: {}" , s . error ()) ? ; let verify = s . ssl () . verify_result () ; if verify != X509VerifyResult :: OK { write ! (f , ": {}" , verify) ? ; } } HandshakeError :: WouldBlock (ref s) => { write ! (f , "the handshake was interrupted: {}" , s . error ()) ? ; let verify = s . ssl () . verify_result () ; if verify != X509VerifyResult :: OK { write ! (f , ": {}" , verify) ? ; } } } Ok (()) } }
};
}
