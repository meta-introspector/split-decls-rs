// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
pub fn main (dir : & Path , _filter : bool , _uncased : bool) { let mut table = TableBuilder :: new () ; let root = PathBuf :: from (env :: var ("CARGO_MANIFEST_DIR") . unwrap_or_else (| _ | String :: new ())) ; for fname in FILES { let path = root . join (format ! ("tz/{fname}")) ; let file = File :: open (& path) . unwrap_or_else (| e | panic ! ("cannot open {}: {e}" , path . display ())) ; for line in BufReader :: new (file) . lines () { let line = strip_comments (line . unwrap ()) ; table . add_line (Line :: new (& line) . unwrap ()) . unwrap () ; } } # [allow (unused_mut)] let mut table = table . build () ; # [cfg (feature = "filter-by-regex")] if _filter { filter :: maybe_filter_timezone_table (& mut table) ; } let timezone_path = dir . join ("timezones.rs") ; let mut timezone_file = File :: create (timezone_path) . unwrap () ; write_timezone_file (& mut timezone_file , & table , _uncased) . unwrap () ; let directory_path = dir . join ("directory.rs") ; let mut directory_file = File :: create (directory_path) . unwrap () ; let version = detect_iana_db_version () ; write_directory_file (& mut directory_file , & table , & version) . unwrap () ; }
};
}
