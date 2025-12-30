// Generated macro for get_clang_cl_resource_dir (function)
macro_rules! Depcrate_utils_helpersget_clang_cl_resource_dir {
() => {
// Module: crate::utils::helpers
// Provides: {"get_clang_cl_resource_dir"}
// Dependencies: {}
# [doc = " Adapted from <https://github.com/llvm/llvm-project/blob/782e91224601e461c019e0a4573bbccc6094fbcd/llvm/cmake/modules/HandleLLVMOptions.cmake#L1058-L1079>"] # [doc = ""] # [doc = " When `clang-cl` is used with instrumentation, we need to add clang's runtime library resource"] # [doc = " directory to the linker flags, otherwise there will be linker errors about the profiler runtime"] # [doc = " missing. This function returns the path to that directory."] pub fn get_clang_cl_resource_dir (builder : & Builder < '_ > , clang_cl_path : & str) -> PathBuf { let mut builtins_locator = command (clang_cl_path) ; builtins_locator . args (["/clang:-print-libgcc-file-name" , "/clang:--rtlib=compiler-rt"]) ; let clang_rt_builtins = builtins_locator . run_capture_stdout (builder) . stdout () ; let clang_rt_builtins = Path :: new (clang_rt_builtins . trim ()) ; assert ! (clang_rt_builtins . exists () , "`clang-cl` must correctly locate the library runtime directory") ; let clang_rt_dir = clang_rt_builtins . parent () . expect ("The clang lib folder should exist") ; clang_rt_dir . to_path_buf () }
};
}
