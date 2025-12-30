// Generated macro for print_rx_max_stream_data_frames (function)
macro_rules! Depcrate_reports_textprint_rx_max_stream_data_frames {
() => {
// Module: crate::reports::text
// Provides: {"print_rx_max_stream_data_frames"}
// Dependencies: {}
fn print_rx_max_stream_data_frames (data_store : & Datastore) { println ! ("### received MAX_STREAM_DATA frames ###") ; if data_store . received_stream_max_data . is_empty () { println ! ("    None") } else { for entry in & data_store . received_stream_max_data { println ! ("    stream={}, total_count={}, first={:?}, last={}" , entry . 0 , entry . 1 . len () , entry . 1 . first () , stringify_last (entry . 1)) ; } } }
};
}
