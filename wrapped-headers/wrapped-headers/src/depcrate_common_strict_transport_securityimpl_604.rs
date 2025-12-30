// Generated macro for impl_604 (impl)
macro_rules! Depcrate_common_strict_transport_securityimpl_604 {
() => {
// Module: crate::common::strict_transport_security
// Provides: {"impl_604"}
// Dependencies: {}
impl StrictTransportSecurity { # [doc = " Create an STS header that includes subdomains"] pub fn including_subdomains (max_age : Duration) -> StrictTransportSecurity { StrictTransportSecurity { max_age : max_age . into () , include_subdomains : true , } } # [doc = " Create an STS header that excludes subdomains"] pub fn excluding_subdomains (max_age : Duration) -> StrictTransportSecurity { StrictTransportSecurity { max_age : max_age . into () , include_subdomains : false , } } # [doc = " Get whether this should include subdomains."] pub fn include_subdomains (& self) -> bool { self . include_subdomains } # [doc = " Get the max-age."] pub fn max_age (& self) -> Duration { self . max_age . into () } }
};
}
