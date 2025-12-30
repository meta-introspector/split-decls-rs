// Generated macro for impl_286 (impl)
macro_rules! Depcrate_dhimpl_286 {
() => {
// Module: crate::dh
// Provides: {"impl_286"}
// Dependencies: {}
impl < T > DhRef < T > where T : HasParams , { to_pem ! { # [doc = " Serializes the parameters into a PEM-encoded PKCS#3 DHparameter structure."] # [doc = ""] # [doc = " The output will have a header of `-----BEGIN DH PARAMETERS-----`."] # [corresponds (PEM_write_bio_DHparams)] params_to_pem , ffi :: PEM_write_bio_DHparams } to_der ! { # [doc = " Serializes the parameters into a DER-encoded PKCS#3 DHparameter structure."] # [corresponds (i2d_DHparams)] params_to_der , ffi :: i2d_DHparams } # [doc = " Validates DH parameters for correctness"] # [corresponds (DH_check_key)] pub fn check_key (& self) -> Result < bool , ErrorStack > { unsafe { let mut codes = 0 ; cvt (ffi :: DH_check (self . as_ptr () , & mut codes)) ? ; Ok (codes == 0) } } }
};
}
