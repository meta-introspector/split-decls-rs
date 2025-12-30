// Generated macro for librustc_stamp (function)
macro_rules! Depcrate_utils_build_stamplibrustc_stamp {
() => {
// Module: crate::utils::build_stamp
// Provides: {"librustc_stamp"}
// Dependencies: {}
# [doc = " Cargo's output path for librustc in a given stage, compiled by a particular"] # [doc = " `build_compiler` for the specified target."] pub fn librustc_stamp (builder : & Builder < '_ > , build_compiler : Compiler , target : TargetSelection ,) -> BuildStamp { BuildStamp :: new (& builder . cargo_out (build_compiler , Mode :: Rustc , target)) . with_prefix ("librustc") }
};
}
