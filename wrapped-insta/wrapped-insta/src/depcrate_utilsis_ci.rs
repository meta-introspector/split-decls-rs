// Generated macro for is_ci (function)
macro_rules! Depcrate_utilsis_ci {
() => {
// Module: crate::utils
// Provides: {"is_ci"}
// Dependencies: {}
# [doc = " Are we running in in a CI environment?"] pub fn is_ci () -> bool { match env :: var ("CI") . ok () . as_deref () { Some ("false") | Some ("0") | Some ("") => false , None => env :: var ("TF_BUILD") . is_ok () , Some (_) => true , } }
};
}
