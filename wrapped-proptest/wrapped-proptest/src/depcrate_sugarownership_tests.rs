// Generated macro for ownership_tests (module)
macro_rules! Depcrate_sugarownership_tests {
() => {
// Module: crate::sugar
// Provides: {"ownership_tests"}
// Dependencies: {}
# [cfg (test)] mod ownership_tests { # [cfg (feature = "std")] proptest ! { # [test] fn accept_ref_arg (ref s in "[0-9]") { use crate :: std_facade :: String ; fn assert_string (_s : & String) { } assert_string (s) ; } # [test] fn accept_move_arg (s in "[0-9]") { use crate :: std_facade :: String ; fn assert_string (_s : String) { } assert_string (s) ; } } # [derive (Debug)] struct NotClone () ; const MK : fn () -> NotClone = NotClone ; proptest ! { # [test] fn accept_noclone_arg (nc in MK) { let _nc2 : NotClone = nc ; } # [test] fn accept_noclone_ref_arg (ref nc in MK) { let _nc2 : & NotClone = nc ; } } }
};
}
