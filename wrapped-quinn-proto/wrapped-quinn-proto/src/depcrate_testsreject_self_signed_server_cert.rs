// Generated macro for reject_self_signed_server_cert (function)
macro_rules! Depcrate_testsreject_self_signed_server_cert {
() => {
// Module: crate::tests
// Provides: {"reject_self_signed_server_cert"}
// Dependencies: {}
# [test] fn reject_self_signed_server_cert () { let _guard = subscribe () ; let mut pair = Pair :: default () ; info ! ("connecting") ; let mut cert = rcgen :: CertificateParams :: new (["localhost" . into ()]) . unwrap () ; let mut issuer = rcgen :: DistinguishedName :: new () ; issuer . push (rcgen :: DnType :: OrganizationName , "Crazy Quinn's House of Certificates" ,) ; cert . distinguished_name = issuer ; let cert = cert . self_signed (& rcgen :: KeyPair :: generate () . unwrap ()) . unwrap () ; let client_ch = pair . begin_connect (client_config_with_certs (vec ! [cert . into ()])) ; pair . drive () ; assert_matches ! (pair . client_conn_mut (client_ch) . poll () , Some (Event :: ConnectionLost { reason : ConnectionError :: TransportError (ref error) }) if error . code == TransportErrorCode :: crypto (AlertDescription :: UnknownCA . into ())) ; }
};
}
