// Generated macro for push_dict_out (function)
macro_rules! Depcrate_inflate_streampush_dict_out {
() => {
// Module: crate::inflate::stream
// Provides: {"push_dict_out"}
// Dependencies: {}
fn push_dict_out (state : & mut InflateState , next_out : & mut & mut [u8]) -> usize { let n = cmp :: min (state . dict_avail , next_out . len ()) ; (next_out [.. n]) . copy_from_slice (& state . dict [state . dict_ofs .. state . dict_ofs + n]) ; * next_out = & mut mem :: take (next_out) [n ..] ; state . dict_avail -= n ; state . dict_ofs = (state . dict_ofs + (n)) & (TINFL_LZ_DICT_SIZE - 1) ; n }
};
}
