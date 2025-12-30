// Generated macro for parse_xid_properties (function)
macro_rules! Depcrate_parseparse_xid_properties {
() => {
// Module: crate::parse
// Provides: {"parse_xid_properties"}
// Dependencies: {}
pub fn parse_xid_properties (ucd_dir : & Path) -> Properties { let filename = "DerivedCoreProperties.txt" ; let path = ucd_dir . join (filename) ; let contents = fs :: read_to_string (path) . unwrap_or_else (| err | { let suggestion = "Download from https://www.unicode.org/Public/latest/ucd/UCD.zip and unzip." ; let _ = writeln ! (io :: stderr () , "{}: {err}\n{suggestion}" , ucd_dir . display ()) ; process :: exit (1) ; }) ; let mut properties = Properties { unicode_version : parse_unicode_version (filename , & contents) , xid_start : Set :: new () , xid_continue : Set :: new () , } ; for (i , line) in contents . lines () . enumerate () { if line . starts_with ('#') || line . trim () . is_empty () { continue ; } let (lo , hi , name) = parse_line (line) . unwrap_or_else (| | { let _ = writeln ! (io :: stderr () , "{filename} line {i} is unexpected:\n{line}") ; process :: exit (1) ; }) ; let set = match name { "XID_Start" => & mut properties . xid_start , "XID_Continue" => & mut properties . xid_continue , _ => continue , } ; set . extend (lo ..= hi) ; } properties }
};
}
