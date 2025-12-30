// Generated macro for detect_iana_db_version (function)
macro_rules! Depcratedetect_iana_db_version {
() => {
// Module: crate
// Provides: {"detect_iana_db_version"}
// Dependencies: {}
fn detect_iana_db_version () -> String { let root = env :: var ("CARGO_MANIFEST_DIR") . expect ("no Cargo build context") ; let path = Path :: new (& root) . join (Path :: new ("tz/NEWS")) ; let file = File :: open (path) . expect ("failed to open file") ; let mut lines = BufReader :: new (file) . lines () ; while let Some (Ok (line)) = lines . next () { let line = match line . strip_prefix ("Release ") { Some (line) => line , _ => continue , } ; match line . split_once (" - ") { Some ((version , _)) => return version . to_owned () , _ => continue , } } unreachable ! ("no version found") }
};
}
