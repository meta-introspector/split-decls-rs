// Generated macro for tests (module)
macro_rules! Depcrate_events_quictests {
() => {
// Module: crate::events::quic
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use crate :: testing :: * ; # [test] fn packet_header () { let pkt_hdr = make_pkt_hdr (PacketType :: Initial) ; let log_string = r#"{
  "packet_type": "initial",
  "packet_number": 0,
  "version": "1",
  "scil": 8,
  "dcil": 8,
  "scid": "7e37e4dcc6682da8",
  "dcid": "36ce104eee50101c"
}"# ; assert_eq ! (serde_json :: to_string_pretty (& pkt_hdr) . unwrap () , log_string) ; } }
};
}
