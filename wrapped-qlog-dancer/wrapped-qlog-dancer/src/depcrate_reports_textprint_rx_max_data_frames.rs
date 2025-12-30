// Generated macro for print_rx_max_data_frames (function)
macro_rules! Depcrate_reports_textprint_rx_max_data_frames {
() => {
// Module: crate::reports::text
// Provides: {"print_rx_max_data_frames"}
// Dependencies: {}
fn print_rx_max_data_frames (data_store : & Datastore) { println ! ("### received MAX_DATA frames ###") ; if data_store . received_stream_max_data . is_empty () { println ! ("    None") } else { println ! ("    first={:?}, last={:?}" , data_store . received_max_data . first () . unwrap () , data_store . received_max_data . last () . unwrap ()) ; } }
};
}
