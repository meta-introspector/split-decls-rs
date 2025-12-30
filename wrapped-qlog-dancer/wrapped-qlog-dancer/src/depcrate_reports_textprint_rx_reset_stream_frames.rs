// Generated macro for print_rx_reset_stream_frames (function)
macro_rules! Depcrate_reports_textprint_rx_reset_stream_frames {
() => {
// Module: crate::reports::text
// Provides: {"print_rx_reset_stream_frames"}
// Dependencies: {}
fn print_rx_reset_stream_frames (data_store : & Datastore) { println ! ("### received RESET_STREAM frames ###") ; if data_store . received_reset_stream . is_empty () { println ! ("    None") } else { for entry in & data_store . received_reset_stream { println ! ("    stream={}, total_count={}, first={:?}, last={:?}" , entry . 0 , entry . 1 . len () , entry . 1 . first () , stringify_last (entry . 1)) ; } } }
};
}
