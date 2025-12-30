// Generated macro for mz_deflate_oxide (function)
macro_rules! Depcrate_lib_oxidemz_deflate_oxide {
() => {
// Module: crate::lib_oxide
// Provides: {"mz_deflate_oxide"}
// Dependencies: {}
pub fn mz_deflate_oxide (stream_oxide : & mut StreamOxide < Compressor > , flush : i32) -> MZResult { let state : & mut Compressor = { let enum_ref = stream_oxide . state . as_mut () . ok_or (MZError :: Stream) ? ; StateType :: from_enum (enum_ref) } . ok_or (MZError :: Stream) ? ; let next_in = stream_oxide . next_in . as_mut () . ok_or (MZError :: Stream) ? ; let next_out = stream_oxide . next_out . as_mut () . ok_or (MZError :: Stream) ? ; let flush = MZFlush :: new (flush) ? ; let ret = if let Some (compressor) = state . inner . as_mut () { deflate (compressor , next_in , next_out , flush) } else { return Err (MZError :: Param) ; } ; * next_in = & next_in [ret . bytes_consumed as usize ..] ; * next_out = & mut mem :: take (next_out) [ret . bytes_written as usize ..] ; stream_oxide . total_in = stream_oxide . total_in . wrapping_add (ret . bytes_consumed as c_ulong) ; stream_oxide . total_out = stream_oxide . total_out . wrapping_add (ret . bytes_written as c_ulong) ; stream_oxide . adler = state . adler32 () ; ret . into () }
};
}
