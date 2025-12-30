// Generated macro for normalize_untrusted_str (function)
macro_rules! Depcratenormalize_untrusted_str {
() => {
// Module: crate
// Provides: {"normalize_untrusted_str"}
// Dependencies: {}
# [doc = " Normalize the string to avoid any unicode control characters."] # [doc = ""] # [doc = " This is important for untrusted input, as it can contain"] # [doc = " invalid unicode sequences."] pub fn normalize_untrusted_str (s : & str) -> String { renderer :: normalize_whitespace (s) }
};
}
