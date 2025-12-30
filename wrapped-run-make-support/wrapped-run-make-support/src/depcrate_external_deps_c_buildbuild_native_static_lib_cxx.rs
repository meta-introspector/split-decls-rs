// Generated macro for build_native_static_lib_cxx (function)
macro_rules! Depcrate_external_deps_c_buildbuild_native_static_lib_cxx {
() => {
// Module: crate::external_deps::c_build
// Provides: {"build_native_static_lib_cxx"}
// Dependencies: {}
# [doc = " Builds a static lib (`.lib` on Windows MSVC and `.a` for the rest) with the given name."] # [doc = " Built from a C++ file."] # [track_caller] pub fn build_native_static_lib_cxx (lib_name : & str) -> PathBuf { let obj_file = if is_windows_msvc () { format ! ("{lib_name}") } else { format ! ("{lib_name}.o") } ; let src = format ! ("{lib_name}.cpp") ; let lib_path = static_lib_name (lib_name) ; if is_windows_msvc () { cxx () . arg ("-EHs") . arg ("-c") . out_exe (& obj_file) . input (src) . run () ; } else { cxx () . arg ("-c") . out_exe (& obj_file) . input (src) . run () ; } ; let obj_file = if is_windows_msvc () { PathBuf :: from (format ! ("{lib_name}.obj")) } else { PathBuf :: from (format ! ("{lib_name}.o")) } ; llvm_ar () . obj_to_ar () . output_input (& lib_path , & obj_file) . run () ; path (lib_path) }
};
}
