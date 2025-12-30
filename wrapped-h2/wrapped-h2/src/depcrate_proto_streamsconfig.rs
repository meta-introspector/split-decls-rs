// Generated macro for Config (struct)
macro_rules! Depcrate_proto_streamsConfig {
() => {
// Module: crate::proto::streams
// Provides: {"Config"}
// Dependencies: {}
# [derive (Debug)] pub struct Config { # [doc = " Initial maximum number of locally initiated streams."] # [doc = " After receiving a Settings frame from the remote peer,"] # [doc = " the connection will overwrite this value with the"] # [doc = " MAX_CONCURRENT_STREAMS specified in the frame."] pub initial_max_send_streams : usize , # [doc = " Max amount of DATA bytes to buffer per stream."] pub local_max_buffer_size : usize , # [doc = " The stream ID to start the next local stream with"] pub local_next_stream_id : StreamId , # [doc = " If the local peer is willing to receive push promises"] pub local_push_enabled : bool , # [doc = " If extended connect protocol is enabled."] pub extended_connect_protocol_enabled : bool , # [doc = " How long a locally reset stream should ignore frames"] pub local_reset_duration : Duration , # [doc = " Maximum number of locally reset streams to keep at a time"] pub local_reset_max : usize , # [doc = " Maximum number of remotely reset \"pending accept\" streams to keep at a"] # [doc = " time. Going over this number results in a connection error."] pub remote_reset_max : usize , # [doc = " Initial window size of remote initiated streams"] pub remote_init_window_sz : WindowSize , # [doc = " Maximum number of remote initiated streams"] pub remote_max_initiated : Option < usize > , # [doc = " Maximum number of locally reset streams due to protocol error across"] # [doc = " the lifetime of the connection."] # [doc = ""] # [doc = " When this gets exceeded, we issue GOAWAYs."] pub local_max_error_reset_streams : Option < usize > , }
};
}
