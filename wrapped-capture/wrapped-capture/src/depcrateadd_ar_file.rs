// Generated macro for add_ar_file (function)
macro_rules! Depcrateadd_ar_file {
() => {
// Module: crate
// Provides: {"add_ar_file"}
// Dependencies: {}
fn add_ar_file < W : std :: io :: Write > (ar : & mut tar :: Builder < W > , path : & Path , contents : & str) { let mut header = tar :: Header :: new_gnu () ; header . set_entry_type (tar :: EntryType :: file ()) ; header . set_mode (0o644) ; header . set_size (contents . len () as u64) ; header . set_mtime (123456789) ; header . set_cksum () ; ar . append_data (& mut header , path , contents . as_bytes ()) . unwrap () ; }
};
}
