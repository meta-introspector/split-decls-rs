// Generated macro for mz_deflate_end_oxide (function)
macro_rules! Depcrate_lib_oxidemz_deflate_end_oxide {
() => {
// Module: crate::lib_oxide
// Provides: {"mz_deflate_end_oxide"}
// Dependencies: {}
# [doc = " Free the inner compression state."] # [doc = ""] # [doc = " Currently always returns `MZStatus::Ok`."] pub fn mz_deflate_end_oxide (stream_oxide : & mut StreamOxide < Compressor >) -> MZResult { stream_oxide . state = None ; Ok (MZStatus :: Ok) }
};
}
