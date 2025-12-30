// Generated macro for print_tx_stream_frames (function)
macro_rules! Depcrate_reports_textprint_tx_stream_frames {
() => {
// Module: crate::reports::text
// Provides: {"print_tx_stream_frames"}
// Dependencies: {}
fn print_tx_stream_frames (data_store : & Datastore) { println ! ("### sent STREAM frames ###") ; if data_store . sent_stream_frames . is_empty () { println ! ("    None") } else { for entry in & data_store . sent_stream_frames { let total = match entry . 1 . last () { Some ((_ , QuicFrame :: Stream { offset , length , .. })) => { format ! ("{}" , offset + length) } , _ => "n/a" . to_string () , } ; println ! ("    stream={}, total_count={}, first={:?}, last={:?}, total_length={}" , entry . 0 , entry . 1 . len () , entry . 1 . first () , stringify_last (entry . 1) , total) ; } } }
};
}
