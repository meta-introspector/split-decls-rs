// Generated macro for status_callbacks (function)
macro_rules! Depcrate_ssl_teststatus_callbacks {
() => {
// Module: crate::ssl::test
// Provides: {"status_callbacks"}
// Dependencies: {}
# [test] # [cfg (not (any (boringssl , awslc)))] fn status_callbacks () { static CALLED_BACK_SERVER : AtomicBool = AtomicBool :: new (false) ; static CALLED_BACK_CLIENT : AtomicBool = AtomicBool :: new (false) ; let mut server = Server :: builder () ; server . ctx () . set_status_callback (| ssl | { CALLED_BACK_SERVER . store (true , Ordering :: SeqCst) ; let response = OcspResponse :: create (OcspResponseStatus :: UNAUTHORIZED , None) . unwrap () ; let response = response . to_der () . unwrap () ; ssl . set_ocsp_status (& response) . unwrap () ; Ok (true) }) . unwrap () ; let server = server . build () ; let mut client = server . client () ; client . ctx () . set_status_callback (| ssl | { CALLED_BACK_CLIENT . store (true , Ordering :: SeqCst) ; let response = OcspResponse :: from_der (ssl . ocsp_status () . unwrap ()) . unwrap () ; assert_eq ! (response . status () , OcspResponseStatus :: UNAUTHORIZED) ; Ok (true) }) . unwrap () ; let mut client = client . build () . builder () ; client . ssl () . set_status_type (StatusType :: OCSP) . unwrap () ; client . connect () ; assert ! (CALLED_BACK_SERVER . load (Ordering :: SeqCst)) ; assert ! (CALLED_BACK_CLIENT . load (Ordering :: SeqCst)) ; }
};
}
