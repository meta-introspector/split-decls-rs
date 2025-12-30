// Generated macro for extract_version_range (function)
macro_rules! Depcrate_directivesextract_version_range {
() => {
// Module: crate::directives
// Provides: {"extract_version_range"}
// Dependencies: {}
# [doc = " Takes a directive of the form `\"<version1> [- <version2>]\"`, returns the numeric representation"] # [doc = " of `<version1>` and `<version2>` as tuple: `(<version1>, <version2>)`."] # [doc = ""] # [doc = " If the `<version2>` part is omitted, the second component of the tuple is the same as"] # [doc = " `<version1>`."] fn extract_version_range < 'a , F , VersionTy : Clone > (line : & 'a str , parse : F ,) -> Option < (VersionTy , VersionTy) > where F : Fn (& 'a str) -> Option < VersionTy > , { let mut splits = line . splitn (2 , "- ") . map (str :: trim) ; let min = splits . next () . unwrap () ; if min . ends_with ('-') { return None ; } let max = splits . next () ; if min . is_empty () { return None ; } let min = parse (min) ? ; let max = match max { Some ("") => return None , Some (max) => parse (max) ? , _ => min . clone () , } ; Some ((min , max)) }
};
}
