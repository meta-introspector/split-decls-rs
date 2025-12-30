// Generated macro for mz_inflate_init2_oxide (function)
macro_rules! Depcrate_lib_oxidemz_inflate_init2_oxide {
() => {
// Module: crate::lib_oxide
// Provides: {"mz_inflate_init2_oxide"}
// Dependencies: {}
pub fn mz_inflate_init2_oxide (stream_oxide : & mut StreamOxide < InflateState > , window_bits : i32 ,) -> MZResult { if invalid_window_bits (window_bits) { return Err (MZError :: Param) ; } stream_oxide . adler = 0 ; stream_oxide . total_in = 0 ; stream_oxide . total_out = 0 ; stream_oxide . state = Some (Box :: new (InternalState :: Inflate (InflateState :: new_boxed_with_window_bits (window_bits) ,))) ; Ok (MZStatus :: Ok) }
};
}
