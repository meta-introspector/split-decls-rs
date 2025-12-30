// Generated macro for print_tx_max_data_frames (function)
macro_rules! Depcrate_reports_textprint_tx_max_data_frames {
() => {
// Module: crate::reports::text
// Provides: {"print_tx_max_data_frames"}
// Dependencies: {}
fn print_tx_max_data_frames (data_store : & Datastore) { println ! ("### sent MAX_DATA frames ###") ; println ! ("   total_count={}, first={:?}, last={:?}" , data_store . sent_max_data . len () , data_store . sent_max_data . first () . unwrap () , data_store . sent_max_data . last () . unwrap ()) ; }
};
}
