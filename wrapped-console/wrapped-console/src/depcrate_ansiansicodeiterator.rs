// Generated macro for AnsiCodeIterator (struct)
macro_rules! Depcrate_ansiAnsiCodeIterator {
() => {
// Module: crate::ansi
// Provides: {"AnsiCodeIterator"}
// Dependencies: {}
# [doc = " An iterator over ansi codes in a string."] # [doc = ""] # [doc = " This type can be used to scan over ansi codes in a string."] # [doc = " It yields tuples in the form `(s, is_ansi)` where `s` is a slice of"] # [doc = " the original string and `is_ansi` indicates if the slice contains"] # [doc = " ansi codes or string values."] pub struct AnsiCodeIterator < 'a > { s : & 'a str , pending_item : Option < (& 'a str , bool) > , last_idx : usize , cur_idx : usize , iter : Matches < 'a > , }
};
}
