// Generated macro for build_native_static_lib_optimized (function)
macro_rules! Depcrate_external_deps_c_buildbuild_native_static_lib_optimized {
() => {
// Module: crate::external_deps::c_build
// Provides: {"build_native_static_lib_optimized"}
// Dependencies: {}
# [doc = " Builds an optimized static lib (`.lib` on Windows MSVC and `.a` for the rest) with the given name."] # [doc = " Built from a C file."] # [track_caller] pub fn build_native_static_lib_optimized (lib_name : & str) -> PathBuf { build_native_static_lib_internal (lib_name , true) }
};
}
