// Generated macro for sort_bindings (function)
macro_rules! Depcratesort_bindings {
() => {
// Module: crate
// Provides: {"sort_bindings"}
// Dependencies: {}
fn sort_bindings (file_name : & str) -> Result < () , Box < dyn Error > > { let mut f = fs :: File :: options () . read (true) . write (true) . open (file_name) ? ; let mut bindings = String :: new () ; f . read_to_string (& mut bindings) ? ; f . set_len (0) ? ; f . seek (SeekFrom :: Start (0)) ? ; let mut lines = bindings . split_inclusive ('\n') ; for line in & mut lines { f . write (line . as_bytes ()) ? ; if line . contains ("--filter") { break ; } } let mut bindings = Vec :: new () ; for line in & mut lines { if ! line . trim () . is_empty () { bindings . push (line) ; } } bindings . sort_by (| a , b | a . to_lowercase () . cmp (& b . to_lowercase ())) ; for line in bindings { f . write (line . as_bytes ()) ? ; } Ok (()) }
};
}
