// Generated macro for source_file_names (function)
macro_rules! Depcratesource_file_names {
() => {
// Module: crate
// Provides: {"source_file_names"}
// Dependencies: {}
fn source_file_names < P : AsRef < Path > > (dir : P) -> Result < Vec < String > > { let mut names = Vec :: new () ; let mut failures = Vec :: new () ; for entry in fs :: read_dir (dir) ? { let entry = entry ? ; if ! entry . file_type () ? . is_file () { continue ; } let file_name = entry . file_name () ; if file_name == "mod.rs" || file_name == "lib.rs" || file_name == "main.rs" { continue ; } let path = Path :: new (& file_name) ; if path . extension () == Some (OsStr :: new ("rs")) { match file_name . into_string () { Ok (mut utf8) => { utf8 . truncate (utf8 . len () - ".rs" . len ()) ; names . push (utf8) ; } Err (non_utf8) => { failures . push (non_utf8) ; } } } } failures . sort () ; if let Some (failure) = failures . into_iter () . next () { return Err (Error :: Utf8 (failure)) ; } if names . is_empty () { return Err (Error :: Empty) ; } names . sort () ; Ok (names) }
};
}
