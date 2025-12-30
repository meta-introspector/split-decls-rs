// Generated macro for impl_286 (impl)
macro_rules! Depcrate_ansiimpl_286 {
() => {
// Module: crate::ansi
// Provides: {"impl_286"}
// Dependencies: {}
impl < 'a > AnsiCodeIterator < 'a > { # [doc = " Creates a new ansi code iterator."] pub fn new (s : & 'a str) -> AnsiCodeIterator < 'a > { AnsiCodeIterator { s , pending_item : None , last_idx : 0 , cur_idx : 0 , iter : Matches :: new (s) , } } # [doc = " Returns the string slice up to the current match."] pub fn current_slice (& self) -> & str { & self . s [.. self . cur_idx] } # [doc = " Returns the string slice from the current match to the end."] pub fn rest_slice (& self) -> & str { & self . s [self . cur_idx ..] } }
};
}
