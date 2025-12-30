// Generated macro for read_in_full_ignore_missing (function)
macro_rules! Depcrate_search_patternread_in_full_ignore_missing {
() => {
// Module: crate::search::pattern
// Provides: {"read_in_full_ignore_missing"}
// Dependencies: {}
fn read_in_full_ignore_missing (path : & Path , follow_symlinks : bool , buf : & mut Vec < u8 >) -> std :: io :: Result < bool > { buf . clear () ; let file = if follow_symlinks { std :: fs :: File :: open (path) } else { gix_features :: fs :: open_options_no_follow () . read (true) . open (path) } ; Ok (match file { Ok (mut file) => { if let Err (err) = file . read_to_end (buf) { if io_err_is_dir (& err) { false } else { return Err (err) ; } } else { true } } Err (err) if err . kind () == std :: io :: ErrorKind :: NotFound || io_err_is_dir (& err) => false , Err (err) => return Err (err) , }) }
};
}
