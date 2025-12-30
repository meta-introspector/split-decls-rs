// Generated macro for assert_has_not_installed_exe (function)
macro_rules! Depcrate_installassert_has_not_installed_exe {
() => {
// Module: crate::install
// Provides: {"assert_has_not_installed_exe"}
// Dependencies: {}
# [track_caller] pub fn assert_has_not_installed_exe < P : AsRef < Path > > (path : P , name : & 'static str) { assert ! (! check_has_installed_exe (path , name)) ; }
};
}
