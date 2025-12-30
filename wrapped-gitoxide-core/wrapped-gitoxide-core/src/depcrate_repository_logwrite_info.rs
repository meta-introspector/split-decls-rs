// Generated macro for write_info (function)
macro_rules! Depcrate_repository_logwrite_info {
() => {
// Module: crate::repository::log
// Provides: {"write_info"}
// Dependencies: {}
fn write_info (repo : & gix :: Repository , mut out : impl std :: io :: Write , info : & gix :: traverse :: commit :: Info ,) -> Result < () , std :: io :: Error > { let commit = repo . find_commit (info . id) . unwrap () ; let message = commit . message_raw_sloppy () ; let title = message . lines () . next () ; writeln ! (out , "{} {}" , info . id . to_hex_with_len (8) , title . map_or_else (|| "<no message>" . into () , BString :: from)) ? ; Ok (()) }
};
}
