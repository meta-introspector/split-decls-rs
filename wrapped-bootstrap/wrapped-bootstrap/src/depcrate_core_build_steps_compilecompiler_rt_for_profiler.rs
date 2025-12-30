// Generated macro for compiler_rt_for_profiler (function)
macro_rules! Depcrate_core_build_steps_compilecompiler_rt_for_profiler {
() => {
// Module: crate::core::build_steps::compile
// Provides: {"compiler_rt_for_profiler"}
// Dependencies: {}
# [doc = " Tries to find LLVM's `compiler-rt` source directory, for building `library/profiler_builtins`."] # [doc = ""] # [doc = " Normally it lives in the `src/llvm-project` submodule, but if we will be using a"] # [doc = " downloaded copy of CI LLVM, then we try to use the `compiler-rt` sources from"] # [doc = " there instead, which lets us avoid checking out the LLVM submodule."] fn compiler_rt_for_profiler (builder : & Builder < '_ >) -> PathBuf { if builder . config . llvm_from_ci { builder . config . maybe_download_ci_llvm () ; let ci_llvm_compiler_rt = builder . config . ci_llvm_root () . join ("compiler-rt") ; if ci_llvm_compiler_rt . exists () { return ci_llvm_compiler_rt ; } } builder . require_submodule ("src/llvm-project" , { Some ("The `build.profiler` config option requires `compiler-rt` sources from LLVM.") }) ; builder . src . join ("src/llvm-project/compiler-rt") }
};
}
