// Generated macro for impl_227 (impl)
macro_rules! Depcrate_inflate_streamimpl_227 {
() => {
// Module: crate::inflate::stream
// Provides: {"impl_227"}
// Dependencies: {}
impl ResetPolicy for ZeroReset { # [inline] fn reset (& self , state : & mut InflateState) { MinReset . reset (state) ; state . dict = [0 ; TINFL_LZ_DICT_SIZE] ; } }
};
}
