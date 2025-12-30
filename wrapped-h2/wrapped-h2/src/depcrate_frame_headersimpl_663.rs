// Generated macro for impl_663 (impl)
macro_rules! Depcrate_frame_headersimpl_663 {
() => {
// Module: crate::frame::headers
// Provides: {"impl_663"}
// Dependencies: {}
impl EncodingHeaderBlock { fn encode < F > (mut self , head : & Head , dst : & mut EncodeBuf < '_ > , f : F) -> Option < Continuation > where F : FnOnce (& mut EncodeBuf < '_ >) , { let head_pos = dst . get_ref () . len () ; head . encode (0 , dst) ; let payload_pos = dst . get_ref () . len () ; f (dst) ; let continuation = if self . hpack . len () > dst . remaining_mut () { dst . put ((& mut self . hpack) . take (dst . remaining_mut ())) ; Some (Continuation { stream_id : head . stream_id () , header_block : self , }) } else { dst . put_slice (& self . hpack) ; None } ; let payload_len = (dst . get_ref () . len () - payload_pos) as u64 ; let payload_len_be = payload_len . to_be_bytes () ; assert ! (payload_len_be [0 .. 5] . iter () . all (| b | * b == 0)) ; (dst . get_mut () [head_pos .. head_pos + 3]) . copy_from_slice (& payload_len_be [5 ..]) ; if continuation . is_some () { debug_assert ! (dst . get_ref () [head_pos + 4] & END_HEADERS == END_HEADERS) ; dst . get_mut () [head_pos + 4] -= END_HEADERS ; } continuation } }
};
}
