// Generated macro for prompt_send_datagram (function)
macro_rules! Depcrate_prompts_h3prompt_send_datagram {
() => {
// Module: crate::prompts::h3
// Provides: {"prompt_send_datagram"}
// Dependencies: {}
pub fn prompt_send_datagram (with_quarter_stream : bool) -> InquireResult < Action > { if with_quarter_stream { let stream_id = h3 :: prompt_varint ("stream ID to be quartered:") ? ; let quarter_stream_id = stream_id / 4 ; let payload = Text :: new ("payload bytes:") . prompt () ? ; let len = octets :: varint_len (quarter_stream_id) + payload . len () ; let mut d = vec ! [0 ; len] ; let mut b = octets :: OctetsMut :: with_slice (& mut d) ; b . put_varint (quarter_stream_id) . unwrap () ; b . put_bytes (payload . as_bytes ()) . unwrap () ; Ok (Action :: SendDatagram { payload : d }) } else { let payload_str = Text :: new ("payload bytes:") . prompt () ? ; Ok (Action :: SendDatagram { payload : payload_str . as_bytes () . to_owned () , }) } }
};
}
