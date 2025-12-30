// Generated macro for libstd_stamp (function)
macro_rules! Depcrate_utils_build_stamplibstd_stamp {
() => {
// Module: crate::utils::build_stamp
// Provides: {"libstd_stamp"}
// Dependencies: {}
# [doc = " Cargo's output path for the standard library in a given stage, compiled"] # [doc = " by a particular `build_compiler` for the specified `target`."] pub fn libstd_stamp (builder : & Builder < '_ > , build_compiler : Compiler , target : TargetSelection ,) -> BuildStamp { BuildStamp :: new (& builder . cargo_out (build_compiler , Mode :: Std , target)) . with_prefix ("libstd") }
};
}
