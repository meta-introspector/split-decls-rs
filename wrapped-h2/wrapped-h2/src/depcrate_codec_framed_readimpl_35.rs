// Generated macro for impl_35 (impl)
macro_rules! Depcrate_codec_framed_readimpl_35 {
() => {
// Module: crate::codec::framed_read
// Provides: {"impl_35"}
// Dependencies: {}
impl < T > Stream for FramedRead < T > where T : AsyncRead + Unpin , { type Item = Result < Frame , Error > ; fn poll_next (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let span = tracing :: trace_span ! ("FramedRead::poll_next") ; let _e = span . enter () ; loop { tracing :: trace ! ("poll") ; let bytes = match ready ! (Pin :: new (& mut self . inner) . poll_next (cx)) { Some (Ok (bytes)) => bytes , Some (Err (e)) => return Poll :: Ready (Some (Err (map_err (e)))) , None => return Poll :: Ready (None) , } ; tracing :: trace ! (read . bytes = bytes . len ()) ; let Self { ref mut hpack , max_header_list_size , ref mut partial , max_continuation_frames , .. } = * self ; if let Some (frame) = decode_frame (hpack , max_header_list_size , max_continuation_frames , partial , bytes ,) ? { tracing :: debug ! (? frame , "received") ; return Poll :: Ready (Some (Ok (frame))) ; } } } }
};
}
