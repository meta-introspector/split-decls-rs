// Generated macro for impl_154 (impl)
macro_rules! Depcrate_execimpl_154 {
() => {
// Module: crate::exec
// Provides: {"impl_154"}
// Dependencies: {}
impl < 'c > RegularExpression for ExecNoSyncStr < 'c > { type Text = str ; fn slots_len (& self) -> usize { self . 0 . slots_len () } fn next_after_empty (& self , text : & str , i : usize) -> usize { next_utf8 (text . as_bytes () , i) } # [inline (always)] fn shortest_match_at (& self , text : & str , start : usize) -> Option < usize > { self . 0 . shortest_match_at (text . as_bytes () , start) } # [inline (always)] fn is_match_at (& self , text : & str , start : usize) -> bool { self . 0 . is_match_at (text . as_bytes () , start) } # [inline (always)] fn find_at (& self , text : & str , start : usize) -> Option < (usize , usize) > { self . 0 . find_at (text . as_bytes () , start) } # [inline (always)] fn read_captures_at (& self , slots : & mut [Slot] , text : & str , start : usize ,) -> Option < (usize , usize) > { self . 0 . read_captures_at (slots , text . as_bytes () , start) } }
};
}
