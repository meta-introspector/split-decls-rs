// Generated macro for parse_excludes_file (function)
macro_rules! Depcrate_gitignoreparse_excludes_file {
() => {
// Module: crate::gitignore
// Provides: {"parse_excludes_file"}
// Dependencies: {}
# [doc = " Extract git's `core.excludesfile` config setting from the raw file contents"] # [doc = " given."] fn parse_excludes_file (data : & [u8]) -> Option < PathBuf > { use std :: sync :: OnceLock ; use regex_automata :: { meta :: Regex , util :: syntax } ; static RE : OnceLock < Regex > = OnceLock :: new () ; let re = RE . get_or_init (| | { Regex :: builder () . configure (Regex :: config () . utf8_empty (false)) . syntax (syntax :: Config :: new () . utf8 (false)) . build (r#"(?im-u)^\s*excludesfile\s*=\s*"?\s*(\S+?)\s*"?\s*$"#) . unwrap () }) ; let mut caps = re . create_captures () ; re . captures (data , & mut caps) ; let span = caps . get_group (1) ? ; let candidate = & data [span] ; std :: str :: from_utf8 (candidate) . ok () . map (| s | PathBuf :: from (expand_tilde (s))) }
};
}
