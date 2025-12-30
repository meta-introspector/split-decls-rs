// Generated macro for Config (struct)
macro_rules! Depcrate_proto_connectionConfig {
() => {
// Module: crate::proto::connection
// Provides: {"Config"}
// Dependencies: {}
# [derive (Debug , Clone)] pub (crate) struct Config { pub next_stream_id : StreamId , pub initial_max_send_streams : usize , pub max_send_buffer_size : usize , pub reset_stream_duration : Duration , pub reset_stream_max : usize , pub remote_reset_stream_max : usize , pub local_error_reset_streams_max : Option < usize > , pub settings : frame :: Settings , }
};
}
