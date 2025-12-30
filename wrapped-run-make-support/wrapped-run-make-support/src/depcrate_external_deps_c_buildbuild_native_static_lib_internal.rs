// Generated macro for build_native_static_lib_internal (function)
macro_rules! Depcrate_external_deps_c_buildbuild_native_static_lib_internal {
() => {
// Module: crate::external_deps::c_build
// Provides: {"build_native_static_lib_internal"}
// Dependencies: {}
# [track_caller] fn build_native_static_lib_internal (lib_name : & str , optimzed : bool) -> PathBuf { let obj_file = if is_windows_msvc () { format ! ("{lib_name}") } else { format ! ("{lib_name}.o") } ; let src = format ! ("{lib_name}.c") ; let lib_path = static_lib_name (lib_name) ; let mut cc = cc () ; if ! is_windows_msvc () { cc . arg ("-v") ; } if optimzed { cc . optimize () ; } cc . arg ("-c") . out_exe (& obj_file) . input (src) . optimize () . run () ; let obj_file = if is_windows_msvc () { PathBuf :: from (format ! ("{lib_name}.obj")) } else { PathBuf :: from (format ! ("{lib_name}.o")) } ; llvm_ar () . obj_to_ar () . output_input (& lib_path , & obj_file) . run () ; path (lib_path) }
};
}
