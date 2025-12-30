// Generated macro for certificate_check_cb (function)
macro_rules! Depcrate_remote_callbackscertificate_check_cb {
() => {
// Module: crate::remote_callbacks
// Provides: {"certificate_check_cb"}
// Dependencies: {}
extern "C" fn certificate_check_cb (cert : * mut raw :: git_cert , _valid : c_int , hostname : * const c_char , data : * mut c_void ,) -> c_int { let ok = panic :: wrap (| | unsafe { let payload = & mut * (data as * mut RemoteCallbacks < '_ >) ; let callback = match payload . certificate_check { Some (ref mut c) => c , None => return Ok (CertificateCheckStatus :: CertificatePassthrough) , } ; let cert = Binding :: from_raw (cert) ; let hostname = str :: from_utf8 (CStr :: from_ptr (hostname) . to_bytes ()) . unwrap () ; callback (& cert , hostname) }) ; match ok { Some (Ok (CertificateCheckStatus :: CertificateOk)) => 0 , Some (Ok (CertificateCheckStatus :: CertificatePassthrough)) => raw :: GIT_PASSTHROUGH as c_int , Some (Err (e)) => unsafe { e . raw_set_git_error () } , None => { - 1 } } }
};
}
