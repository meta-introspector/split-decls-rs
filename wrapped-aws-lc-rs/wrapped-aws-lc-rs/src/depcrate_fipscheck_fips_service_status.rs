// Generated macro for check_fips_service_status (macro)
macro_rules! Depcrate_fipscheck_fips_service_status {
() => {
// Module: crate::fips
// Provides: {"check_fips_service_status"}
// Dependencies: {}
# [allow (unused_macros)] # [cfg (all (feature = "fips" , debug_assertions))] macro_rules ! check_fips_service_status { ($ function : expr) => { { use $ crate :: fips :: { clear_fips_service_status , get_fips_service_status } ; clear_fips_service_status () ; let result = $ function ; get_fips_service_status () . map (| () | result) } } ; }
};
}
