// Generated macro for impl_605 (impl)
macro_rules! Depcrate_frame_dataimpl_605 {
() => {
// Module: crate::frame::data
// Provides: {"impl_605"}
// Dependencies: {}
impl Data < Bytes > { pub (crate) fn load (head : Head , mut payload : Bytes) -> Result < Self , Error > { let flags = DataFlags :: load (head . flag ()) ; if head . stream_id () . is_zero () { return Err (Error :: InvalidStreamId) ; } let pad_len = if flags . is_padded () { let len = util :: strip_padding (& mut payload) ? ; Some (len) } else { None } ; Ok (Data { stream_id : head . stream_id () , data : payload , flags , pad_len , }) } }
};
}
