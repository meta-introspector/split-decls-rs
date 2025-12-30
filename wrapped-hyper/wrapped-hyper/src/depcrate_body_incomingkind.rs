// Generated macro for Kind (enum)
macro_rules! Depcrate_body_incomingKind {
() => {
// Module: crate::body::incoming
// Provides: {"Kind"}
// Dependencies: {}
enum Kind { Empty , # [cfg (all (feature = "http1" , any (feature = "client" , feature = "server")))] Chan { content_length : DecodedLength , want_tx : watch :: Sender , data_rx : mpsc :: Receiver < Result < Bytes , crate :: Error > > , trailers_rx : oneshot :: Receiver < HeaderMap > , } , # [cfg (all (feature = "http2" , any (feature = "client" , feature = "server")))] H2 { content_length : DecodedLength , data_done : bool , ping : ping :: Recorder , recv : h2 :: RecvStream , } , # [cfg (feature = "ffi")] Ffi (crate :: ffi :: UserBody) , }
};
}
