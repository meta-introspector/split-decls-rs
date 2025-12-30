// Generated macro for libgccjit_built_path (function)
macro_rules! Depcrate_core_build_steps_gcclibgccjit_built_path {
() => {
// Module: crate::core::build_steps::gcc
// Provides: {"libgccjit_built_path"}
// Dependencies: {}
# [doc = " Returns the path to a libgccjit.so file in the install directory of GCC."] fn libgccjit_built_path (install_dir : & Path) -> PathBuf { install_dir . join ("lib/libgccjit.so") }
};
}
