// Generated macro for get_windows_msvc_libs (function)
macro_rules! Depcrate_external_deps_c_cxx_compiler_extrasget_windows_msvc_libs {
() => {
// Module: crate::external_deps::c_cxx_compiler::extras
// Provides: {"get_windows_msvc_libs"}
// Dependencies: {}
fn get_windows_msvc_libs () -> Vec < & 'static str > { let mut libs = vec ! ["ws2_32.lib" , "userenv.lib" , "bcrypt.lib" , "ntdll.lib" , "synchronization.lib"] ; if is_win7 () { libs . push ("advapi32.lib") ; } libs }
};
}
