// Generated macro for impl_1441 (impl)
macro_rules! Depcrate_x509_storeimpl_1441 {
() => {
// Module: crate::x509::store
// Provides: {"impl_1441"}
// Dependencies: {}
# [cfg (not (any (boringssl , awslc)))] impl X509LookupRef < HashDir > { # [doc = " Specifies a directory from which certificates and CRLs will be loaded"] # [doc = " on-demand. Must be used with `X509Lookup::hash_dir`."] # [corresponds (X509_LOOKUP_add_dir)] pub fn add_dir (& mut self , name : & str , file_type : SslFiletype) -> Result < () , ErrorStack > { let name = CString :: new (name) . unwrap () ; unsafe { cvt (ffi :: X509_LOOKUP_add_dir (self . as_ptr () , name . as_ptr () , file_type . as_raw () ,)) . map (| _ | ()) } } }
};
}
