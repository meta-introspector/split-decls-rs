// Generated macro for set_fips_service_status_unapproved (function)
macro_rules! Depcrate_fipsset_fips_service_status_unapproved {
() => {
// Module: crate::fips
// Provides: {"set_fips_service_status_unapproved"}
// Dependencies: {}
# [inline] pub (crate) fn set_fips_service_status_unapproved () { # [cfg (all (feature = "fips" , debug_assertions))] indicator :: set_unapproved () ; }
};
}
