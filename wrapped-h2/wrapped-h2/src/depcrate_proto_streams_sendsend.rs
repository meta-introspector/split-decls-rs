// Generated macro for Send (struct)
macro_rules! Depcrate_proto_streams_sendSend {
() => {
// Module: crate::proto::streams::send
// Provides: {"Send"}
// Dependencies: {}
# [doc = " Manages state transitions related to outbound frames."] # [derive (Debug)] pub (super) struct Send { # [doc = " Stream identifier to use for next initialized stream."] next_stream_id : Result < StreamId , StreamIdOverflow > , # [doc = " Any streams with a higher ID are ignored."] # [doc = ""] # [doc = " This starts as MAX, but is lowered when a GOAWAY is received."] # [doc = ""] # [doc = " > After sending a GOAWAY frame, the sender can discard frames for"] # [doc = " > streams initiated by the receiver with identifiers higher than"] # [doc = " > the identified last stream."] max_stream_id : StreamId , # [doc = " Initial window size of locally initiated streams"] init_window_sz : WindowSize , # [doc = " Prioritization layer"] prioritize : Prioritize , is_push_enabled : bool , # [doc = " If extended connect protocol is enabled."] is_extended_connect_protocol_enabled : bool , }
};
}
