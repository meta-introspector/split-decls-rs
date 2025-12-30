// Generated macro for escape_for_toml (function)
macro_rules! Depcrate_utilescape_for_toml {
() => {
// Module: crate::util
// Provides: {"escape_for_toml"}
// Dependencies: {}
# [doc = " Escapes `s` in a way that is suitable for using it as a string literal in TOML syntax."] pub fn escape_for_toml (s : & str) -> String { let s = s . replace ('\\' , r"\\") . replace ('"' , r#"\""#) ; format ! ("\"{s}\"") }
};
}
