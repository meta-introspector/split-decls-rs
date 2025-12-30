// Generated macro for packet_size (function)
macro_rules! Depcrate_tests_utilpacket_size {
() => {
// Module: crate::tests::util
// Provides: {"packet_size"}
// Dependencies: {}
fn packet_size (transmit : & Transmit , buffer : & Bytes) -> usize { if transmit . segment_size . is_some () { panic ! ("This transmit is meant to be split into multiple packets!") ; } buffer . len () }
};
}
