// Generated macro for mz_deflate_reset_oxide (function)
macro_rules! Depcrate_lib_oxidemz_deflate_reset_oxide {
() => {
// Module: crate::lib_oxide
// Provides: {"mz_deflate_reset_oxide"}
// Dependencies: {}
# [doc = " Reset the compressor, so it can be used to compress a new set of data."] # [doc = ""] # [doc = " Returns `MZError::Stream` if the inner stream is missing, otherwise `MZStatus::Ok`."] pub fn mz_deflate_reset_oxide (stream_oxide : & mut StreamOxide < Compressor >) -> MZResult { stream_oxide . total_in = 0 ; stream_oxide . total_out = 0 ; stream_oxide . adler = 0 ; stream_oxide . next_in = None ; stream_oxide . next_out = None ; let state = stream_oxide . state () . ok_or (MZError :: Stream) ? ; state . reset () ; Ok (MZStatus :: Ok) }
};
}
