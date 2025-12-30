// Generated macro for string_excluding_ansi (function)
macro_rules! Depcrate_log_formatstring_excluding_ansi {
() => {
// Module: crate::log::format
// Provides: {"string_excluding_ansi"}
// Dependencies: {}
# [doc = " Returns the given string excluding ANSI control sequences."] fn string_excluding_ansi (s : & str) -> String { let re = Regex :: new (r"\x1b\[[0-9;]*m") . unwrap () ; re . replace_all (s , "") . to_string () }
};
}
