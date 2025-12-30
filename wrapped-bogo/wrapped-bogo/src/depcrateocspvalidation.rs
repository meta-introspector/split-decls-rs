// Generated macro for OcspValidation (enum)
macro_rules! DepcrateOcspValidation {
() => {
// Module: crate
// Provides: {"OcspValidation"}
// Dependencies: {}
# [derive (Clone , Copy , Debug , Default)] enum OcspValidation { # [doc = " Totally ignore `ocsp_response` value"] # [default] None , # [doc = " Return an error (irrespective of `ocsp_response` value)"] Reject , }
};
}
