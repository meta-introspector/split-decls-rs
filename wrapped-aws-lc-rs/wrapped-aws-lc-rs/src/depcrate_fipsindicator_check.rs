// Generated macro for indicator_check (macro)
macro_rules! Depcrate_fipsindicator_check {
() => {
// Module: crate::fips
// Provides: {"indicator_check"}
// Dependencies: {}
macro_rules ! indicator_check { ($ function : expr) => { { # [cfg (all (feature = "fips" , debug_assertions))] { use crate :: fips :: { service_indicator_after_call , service_indicator_before_call } ; let before = service_indicator_before_call () ; let result = $ function ; let after = service_indicator_after_call () ; if before == after { crate :: fips :: indicator :: set_unapproved () ; result } else { crate :: fips :: indicator :: set_approved () ; result } } # [cfg (any (not (feature = "fips") , not (debug_assertions)))] { $ function } } } ; }
};
}
