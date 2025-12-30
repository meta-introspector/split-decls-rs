// Generated macro for build_native_dynamic_lib (function)
macro_rules! Depcrate_external_deps_c_buildbuild_native_dynamic_lib {
() => {
// Module: crate::external_deps::c_build
// Provides: {"build_native_dynamic_lib"}
// Dependencies: {}
# [doc = " Builds a dynamic lib. The filename is computed in a target-dependent manner, relying on"] # [doc = " [`std::env::consts::DLL_PREFIX`] and [`std::env::consts::DLL_EXTENSION`]."] # [track_caller] pub fn build_native_dynamic_lib (lib_name : & str) -> PathBuf { let obj_file = if is_windows_msvc () { format ! ("{lib_name}") } else { format ! ("{lib_name}.o") } ; let src = format ! ("{lib_name}.c") ; let lib_path = dynamic_lib_name (lib_name) ; if is_windows_msvc () { cc () . arg ("-c") . out_exe (& obj_file) . input (src) . run () ; } else { cc () . arg ("-v") . arg ("-c") . out_exe (& obj_file) . input (src) . run () ; } ; let obj_file = if is_windows_msvc () { format ! ("{lib_name}.obj") } else { format ! ("{lib_name}.o") } ; if is_windows_msvc () { let out_arg = format ! ("-out:{lib_path}") ; cc () . input (& obj_file) . args (& ["-link" , "-dll" , & out_arg]) . run () ; } else if is_darwin () { cc () . out_exe (& lib_path) . input (& obj_file) . args (& ["-dynamiclib" , "-Wl,-dylib"]) . run () ; } else if is_windows () { cc () . out_exe (& lib_path) . input (& obj_file) . args (& ["-shared" , & format ! ("-Wl,--out-implib={lib_path}.a")]) . run () ; } else { cc () . out_exe (& lib_path) . input (& obj_file) . arg ("-shared") . run () ; } path (lib_path) }
};
}
