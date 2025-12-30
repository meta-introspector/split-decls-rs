// Generated macro for windows_char_len (function)
macro_rules! Depcratewindows_char_len {
() => {
// Module: crate
// Provides: {"windows_char_len"}
// Dependencies: {}
# [cfg (any (windows , test))] fn windows_char_len (s : & OsStr) -> usize { # [cfg (not (windows))] let len = s . to_string_lossy () . chars () . map (| c | if c as u32 <= 0xFFFF { 1 } else { 2 }) . sum () ; # [cfg (windows)] let len = s . encode_wide () . count () ; len }
};
}
