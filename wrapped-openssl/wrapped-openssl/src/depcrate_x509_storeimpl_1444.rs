// Generated macro for impl_1444 (impl)
macro_rules! Depcrate_x509_storeimpl_1444 {
() => {
// Module: crate::x509::store
// Provides: {"impl_1444"}
// Dependencies: {}
# [cfg (not (any (boringssl , awslc)))] impl X509LookupRef < File > { # [doc = " Specifies a file from which certificates will be loaded"] # [corresponds (X509_load_cert_file)] pub fn load_cert_file < P : AsRef < Path > > (& mut self , file : P , file_type : SslFiletype ,) -> Result < () , ErrorStack > { let file = CString :: new (file . as_ref () . as_os_str () . to_str () . unwrap ()) . unwrap () ; unsafe { cvt (ffi :: X509_load_cert_file (self . as_ptr () , file . as_ptr () , file_type . as_raw () ,)) . map (| _ | ()) } } # [doc = " Specifies a file from which certificate revocation lists will be loaded"] # [corresponds (X509_load_crl_file)] pub fn load_crl_file < P : AsRef < Path > > (& mut self , file : P , file_type : SslFiletype ,) -> Result < i32 , ErrorStack > { let file = CString :: new (file . as_ref () . as_os_str () . to_str () . unwrap ()) . unwrap () ; unsafe { cvt (ffi :: X509_load_crl_file (self . as_ptr () , file . as_ptr () , file_type . as_raw () ,)) } } }
};
}
