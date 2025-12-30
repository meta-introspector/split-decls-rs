// Generated macro for FipsServiceStatus (enum)
macro_rules! Depcrate_fipsFipsServiceStatus {
() => {
// Module: crate::fips
// Provides: {"FipsServiceStatus"}
// Dependencies: {}
# [doc = " The FIPS Module Service Status"] # [allow (dead_code)] # [cfg (all (feature = "fips" , debug_assertions))] # [allow (clippy :: module_name_repetitions)] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub (crate) enum FipsServiceStatus < R > { # [doc = " Indicates that the current thread is using approved FIPS cryptographic services."] Approved (R) , # [doc = " Indicates that the current thread has used non-approved FIPS cryptographic services."] # [doc = " The service indicator status can be reset using `reset_fips_service_status`."] # [doc = " `reset_fips_service_status` will return `NonApprovedMode` if the service used a non-approved"] # [doc = " service, and automatically resets the service status for you."] NonApproved (R) , # [doc = " Indicates that the service indicator is not set"] Unset (R) , }
};
}
