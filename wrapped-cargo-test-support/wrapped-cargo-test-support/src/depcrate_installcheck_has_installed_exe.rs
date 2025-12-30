// Generated macro for check_has_installed_exe (function)
macro_rules! Depcrate_installcheck_has_installed_exe {
() => {
// Module: crate::install
// Provides: {"check_has_installed_exe"}
// Dependencies: {}
fn check_has_installed_exe < P : AsRef < Path > > (path : P , name : & 'static str) -> bool { path . as_ref () . join ("bin") . join (exe (name)) . is_file () }
};
}
