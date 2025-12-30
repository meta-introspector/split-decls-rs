// Generated macro for extra_linker_flags (function)
macro_rules! Depcrate_external_deps_c_cxx_compiler_extrasextra_linker_flags {
() => {
// Module: crate::external_deps::c_cxx_compiler::extras
// Provides: {"extra_linker_flags"}
// Dependencies: {}
pub fn extra_linker_flags () -> Vec < & 'static str > { if is_windows_msvc () { let mut args = get_windows_msvc_libs () ; if is_arm64ec () { args . push ("/MACHINE:ARM64EC") ; } args } else { vec ! [] } }
};
}
