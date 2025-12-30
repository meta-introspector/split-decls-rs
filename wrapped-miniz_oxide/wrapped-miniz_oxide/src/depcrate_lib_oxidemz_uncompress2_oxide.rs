// Generated macro for mz_uncompress2_oxide (function)
macro_rules! Depcrate_lib_oxidemz_uncompress2_oxide {
() => {
// Module: crate::lib_oxide
// Provides: {"mz_uncompress2_oxide"}
// Dependencies: {}
pub fn mz_uncompress2_oxide (stream_oxide : & mut StreamOxide < InflateState > , dest_len : & mut c_ulong ,) -> MZResult { mz_inflate_init_oxide (stream_oxide) ? ; let status = mz_inflate_oxide (stream_oxide , MZFlush :: Finish as i32) ; mz_inflate_end_oxide (stream_oxide) ? ; let empty_in = stream_oxide . next_in . map_or (true , | next_in | next_in . is_empty ()) ; match (status , empty_in) { (Ok (MZStatus :: StreamEnd) , _) => { * dest_len = stream_oxide . total_out ; Ok (MZStatus :: Ok) } (Err (MZError :: Buf) , true) => Err (MZError :: Data) , (status , _) => status , } }
};
}
