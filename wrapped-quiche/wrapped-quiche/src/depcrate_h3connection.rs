// Generated macro for Connection (struct)
macro_rules! Depcrate_h3Connection {
() => {
// Module: crate::h3
// Provides: {"Connection"}
// Dependencies: {}
# [doc = " An HTTP/3 connection."] pub struct Connection { is_server : bool , next_request_stream_id : u64 , next_uni_stream_id : u64 , streams : crate :: stream :: StreamIdHashMap < stream :: Stream > , local_settings : ConnectionSettings , peer_settings : ConnectionSettings , control_stream_id : Option < u64 > , peer_control_stream_id : Option < u64 > , qpack_encoder : qpack :: Encoder , qpack_decoder : qpack :: Decoder , local_qpack_streams : QpackStreams , peer_qpack_streams : QpackStreams , max_push_id : u64 , finished_streams : VecDeque < u64 > , frames_greased : bool , local_goaway_id : Option < u64 > , peer_goaway_id : Option < u64 > , }
};
}
