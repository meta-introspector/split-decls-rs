// Generated macro for mz_compress2_oxide (function)
macro_rules! Depcrate_lib_oxidemz_compress2_oxide {
() => {
// Module: crate::lib_oxide
// Provides: {"mz_compress2_oxide"}
// Dependencies: {}
# [doc = " Try to fully decompress the data provided in the stream struct, with the specified"] # [doc = " level."] # [doc = ""] # [doc = " Returns MZResult::Ok on success."] pub fn mz_compress2_oxide (stream_oxide : & mut StreamOxide < Compressor > , level : i32 , dest_len : & mut c_ulong ,) -> MZResult { mz_deflate_init_oxide (stream_oxide , level) ? ; let status = mz_deflate_oxide (stream_oxide , MZFlush :: Finish as i32) ; mz_deflate_end_oxide (stream_oxide) ? ; match status { Ok (MZStatus :: StreamEnd) => { * dest_len = stream_oxide . total_out ; Ok (MZStatus :: Ok) } Ok (MZStatus :: Ok) => Err (MZError :: Buf) , _ => status , } }
};
}
