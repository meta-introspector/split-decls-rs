// Generated macro for get_fips_service_status (function)
macro_rules! Depcrate_fipsget_fips_service_status {
() => {
// Module: crate::fips
// Provides: {"get_fips_service_status"}
// Dependencies: {}
# [doc = " Retrieve the FIPS module service status."] # [allow (dead_code)] # [cfg (all (feature = "fips" , debug_assertions))] pub (crate) fn get_fips_service_status () -> FipsServiceStatus < () > { if let Some (status) = indicator :: get_status () { if status { FipsServiceStatus :: Approved (()) } else { FipsServiceStatus :: NonApproved (()) } } else { FipsServiceStatus :: Unset (()) } }
};
}
