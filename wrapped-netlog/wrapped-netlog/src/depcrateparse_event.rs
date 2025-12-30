// Generated macro for parse_event (function)
macro_rules! Depcrateparse_event {
() => {
// Module: crate
// Provides: {"parse_event"}
// Dependencies: {}
# [doc = " Parses the provided `event` based on the event type provided in `event_hdr`."] pub fn parse_event (event_hdr : & EventHeader , event : & [u8]) -> Option < Event > { if event_hdr . ty_string . starts_with ("HTTP_") { return http :: parse_event (event_hdr , event) ; } else if event_hdr . ty_string . starts_with ("HTTP2_") { return h2 :: parse_event (event_hdr , event) ; } else if event_hdr . ty_string . starts_with ("HTTP3_") { return h3 :: parse_event (event_hdr , event) ; } else if event_hdr . ty_string . starts_with ("QUIC") { return quic :: parse_event (event_hdr , event) ; } None }
};
}
