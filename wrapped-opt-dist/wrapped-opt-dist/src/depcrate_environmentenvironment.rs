// Generated macro for Environment (struct)
macro_rules! Depcrate_environmentEnvironment {
() => {
// Module: crate::environment
// Provides: {"Environment"}
// Dependencies: {}
# [derive (Builder)] pub struct Environment { host_tuple : String , python_binary : String , # [doc = " The rustc checkout, where the compiler source is located."] checkout_dir : Utf8PathBuf , # [doc = " The main directory where the build occurs. Stage0 rustc and cargo have to be available in"] # [doc = " this directory before `opt-dist` is started."] build_dir : Utf8PathBuf , # [doc = " Directory where the optimization artifacts (PGO/BOLT profiles, etc.)"] # [doc = " will be stored."] artifact_dir : Utf8PathBuf , # [doc = " Path to the host LLVM used to compile LLVM in `src/llvm-project`."] host_llvm_dir : Utf8PathBuf , # [doc = " List of test paths that should be skipped when testing the optimized artifacts."] skipped_tests : Vec < String > , # [doc = " Arguments passed to `rustc-perf --cargo-config <value>` when running benchmarks."] # [builder (default)] benchmark_cargo_config : Vec < String > , # [doc = " Directory containing a pre-built rustc-perf checkout."] # [builder (default)] prebuilt_rustc_perf : Option < Utf8PathBuf > , use_bolt : bool , shared_llvm : bool , run_tests : bool , fast_try_build : bool , build_llvm : bool , # [builder (default)] stage0_root : Option < Utf8PathBuf > , }
};
}
