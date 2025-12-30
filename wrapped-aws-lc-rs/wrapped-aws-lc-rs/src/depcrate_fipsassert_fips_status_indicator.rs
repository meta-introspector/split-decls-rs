// Generated macro for assert_fips_status_indicator (macro)
macro_rules! Depcrate_fipsassert_fips_status_indicator {
() => {
// Module: crate::fips
// Provides: {"assert_fips_status_indicator"}
// Dependencies: {}
# [allow (unused_macros)] # [cfg (all (feature = "fips" , debug_assertions))] macro_rules ! assert_fips_status_indicator { ($ function : expr , $ expect : path) => { assert_fips_status_indicator ! ($ function , $ expect , "unexpected service indicator") } ; ($ function : expr , $ expect : path , $ message : literal) => { { match crate :: fips :: check_fips_service_status ! ($ function) { $ expect (v) => v , _ => panic ! ($ message) , } } } ; }
};
}
