// Generated macro for strip_ansi_codes (function)
macro_rules! Depcrate_ansistrip_ansi_codes {
() => {
// Module: crate::ansi
// Provides: {"strip_ansi_codes"}
// Dependencies: {}
# [doc = " Helper function to strip ansi codes."] # [cfg (feature = "alloc")] pub fn strip_ansi_codes (s : & str) -> Cow < '_ , str > { let mut char_it = s . char_indices () . peekable () ; match find_ansi_code_exclusive (& mut char_it) { Some (_) => { let stripped : String = AnsiCodeIterator :: new (s) . filter_map (| (text , is_ansi) | if is_ansi { None } else { Some (text) }) . collect () ; Cow :: Owned (stripped) } None => Cow :: Borrowed (s) , } }
};
}
