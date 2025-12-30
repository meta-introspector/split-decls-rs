// Generated macro for impl_38 (impl)
macro_rules! Depcrate_linesimpl_38 {
() => {
// Module: crate::lines
// Provides: {"impl_38"}
// Dependencies: {}
impl Decoder for LinesCodec { type Item = String ; type Error = io :: Error ; fn decode (& mut self , src : & mut BytesMut) -> Result < Option < Self :: Item > , Self :: Error > { if src . is_empty () { return Ok (None) ; } let len = match memchr (b'\n' , src) { Some (n) => n , None => { return Ok (None) ; } } ; let mut buf = src . split_to (len) ; debug_assert_eq ! (len , buf . len ()) ; src . advance (1) ; match buf . last () { Some (b'\r') => buf . truncate (len - 1) , None => return Ok (Some (String :: new ())) , _ => { } } try_into_utf8 (buf . freeze ()) } fn decode_eof (& mut self , src : & mut BytesMut) -> Result < Option < Self :: Item > , Self :: Error > { match self . decode (src) ? { Some (frame) => Ok (Some (frame)) , None if src . is_empty () => Ok (None) , None => { let buf = match src . last () { Some (b'\r') => src . split_to (src . len () - 1) , _ => src . split () , } ; if buf . is_empty () { return Ok (None) ; } try_into_utf8 (buf . freeze ()) } } } }
};
}
