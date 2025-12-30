// Generated macro for impl_225 (impl)
macro_rules! Depcrate_inflate_streamimpl_225 {
() => {
// Module: crate::inflate::stream
// Provides: {"impl_225"}
// Dependencies: {}
impl ResetPolicy for MinReset { fn reset (& self , state : & mut InflateState) { state . decompressor () . init () ; state . dict_ofs = 0 ; state . dict_avail = 0 ; state . first_call = true ; state . has_flushed = false ; state . last_status = TINFLStatus :: NeedsMoreInput ; } }
};
}
