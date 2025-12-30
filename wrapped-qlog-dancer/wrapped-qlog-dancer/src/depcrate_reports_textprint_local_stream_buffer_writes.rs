// Generated macro for print_local_stream_buffer_writes (function)
macro_rules! Depcrate_reports_textprint_local_stream_buffer_writes {
() => {
// Module: crate::reports::text
// Provides: {"print_local_stream_buffer_writes"}
// Dependencies: {}
fn print_local_stream_buffer_writes (data_store : & Datastore) { println ! ("### local stream buffer writes ###") ; if data_store . stream_buffer_writes . is_empty () { println ! ("    None") } else { for entry in & data_store . stream_buffer_writes { println ! ("    stream={}, total_count={}, first=(offset={}, length={}), last=(offset={}, length={}), total_length={}" , entry . 0 , entry . 1 . len () , entry . 1 . first () . unwrap () . 1 . offset , entry . 1 . first () . unwrap () . 1 . length , entry . 1 . last () . unwrap () . 1 . offset , entry . 1 . last () . unwrap () . 1 . length , entry . 1 . last () . unwrap () . 1 . offset + entry . 1 . last () . unwrap () . 1 . length ,) ; } } }
};
}
