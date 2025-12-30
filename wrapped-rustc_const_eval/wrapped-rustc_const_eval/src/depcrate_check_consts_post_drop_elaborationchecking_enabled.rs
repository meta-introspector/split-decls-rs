// Generated macro for checking_enabled (function)
macro_rules! Depcrate_check_consts_post_drop_elaborationchecking_enabled {
() => {
// Module: crate::check_consts::post_drop_elaboration
// Provides: {"checking_enabled"}
// Dependencies: {}
# [doc = " Returns `true` if we should use the more precise live drop checker that runs after drop"] # [doc = " elaboration."] pub fn checking_enabled (ccx : & ConstCx < '_ , '_ >) -> bool { if ccx . enforce_recursive_const_stability () { return rustc_allow_const_fn_unstable (ccx . tcx , ccx . body . source . def_id () . expect_local () , sym :: const_precise_live_drops ,) ; } ccx . tcx . features () . const_precise_live_drops () }
};
}
