// Generated macro for impl_232 (impl)
macro_rules! Depcrate_frameimpl_232 {
() => {
// Module: crate::frame
// Provides: {"impl_232"}
// Dependencies: {}
impl Serialize for H3iFrame { fn serialize < S > (& self , s : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { match self { H3iFrame :: QuicheH3 (frame) => { let mut state = s . serialize_struct ("frame" , 1) ? ; let name = frame_name (frame) ; state . serialize_field (name , & SerializableQFrame (frame)) ? ; state . end () } , H3iFrame :: Headers (headers) => { let mut state = s . serialize_struct ("enriched_headers" , 1) ? ; state . serialize_field ("enriched_headers" , headers) ? ; state . end () } , H3iFrame :: ResetStream (reset) => { let mut state = s . serialize_struct ("reset_stream" , 1) ? ; state . serialize_field ("reset_stream" , reset) ? ; state . end () } , } } }
};
}
