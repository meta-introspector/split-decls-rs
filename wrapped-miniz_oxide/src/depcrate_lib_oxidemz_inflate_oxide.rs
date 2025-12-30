// Generated macro for mz_inflate_oxide (function)
macro_rules! Depcrate_lib_oxidemz_inflate_oxide {
() => {
// Module: crate::lib_oxide
// Provides: {"mz_inflate_oxide"}
// Dependencies: {}
pub fn mz_inflate_oxide (stream_oxide : & mut StreamOxide < InflateState > , flush : i32) -> MZResult { let state : & mut InflateState = { let enum_ref = stream_oxide . state . as_mut () . ok_or (MZError :: Stream) ? ; StateType :: from_enum (enum_ref) } . ok_or (MZError :: Stream) ? ; let next_in = stream_oxide . next_in . as_mut () . ok_or (MZError :: Stream) ? ; let next_out = stream_oxide . next_out . as_mut () . ok_or (MZError :: Stream) ? ; let flush = MZFlush :: new (flush) ? ; let ret = inflate (state , next_in , next_out , flush) ; * next_in = & next_in [ret . bytes_consumed as usize ..] ; * next_out = & mut mem :: take (next_out) [ret . bytes_written as usize ..] ; stream_oxide . total_in = stream_oxide . total_in . wrapping_add (ret . bytes_consumed as c_ulong) ; stream_oxide . total_out = stream_oxide . total_out . wrapping_add (ret . bytes_written as c_ulong) ; stream_oxide . adler = state . decompressor () . adler32 () . unwrap_or (0) ; ret . into () }
};
}
