// Generated macro for impl_1545 (impl)
macro_rules! Depcrate_x509impl_1545 {
() => {
// Module: crate::x509
// Provides: {"impl_1545"}
// Dependencies: {}
impl X509 { # [doc = " Returns a new builder."] pub fn builder () -> Result < X509Builder , ErrorStack > { X509Builder :: new () } from_pem ! { # [doc = " Deserializes a PEM-encoded X509 structure."] # [doc = ""] # [doc = " The input should have a header of `-----BEGIN CERTIFICATE-----`."] # [corresponds (PEM_read_bio_X509)] from_pem , X509 , ffi :: PEM_read_bio_X509 } from_der ! { # [doc = " Deserializes a DER-encoded X509 structure."] # [corresponds (d2i_X509)] from_der , X509 , ffi :: d2i_X509 } # [doc = " Deserializes a list of PEM-formatted certificates."] # [corresponds (PEM_read_bio_X509)] pub fn stack_from_pem (pem : & [u8]) -> Result < Vec < X509 > , ErrorStack > { unsafe { ffi :: init () ; let bio = MemBioSlice :: new (pem) ? ; let mut certs = vec ! [] ; loop { let r = ffi :: PEM_read_bio_X509 (bio . as_ptr () , ptr :: null_mut () , None , ptr :: null_mut ()) ; if r . is_null () { let e = ErrorStack :: get () ; if let Some (err) = e . errors () . last () { if err . library_code () == ffi :: ERR_LIB_PEM as libc :: c_int && err . reason_code () == ffi :: PEM_R_NO_START_LINE as libc :: c_int { break ; } } return Err (e) ; } else { certs . push (X509 (r)) ; } } Ok (certs) } } }
};
}
