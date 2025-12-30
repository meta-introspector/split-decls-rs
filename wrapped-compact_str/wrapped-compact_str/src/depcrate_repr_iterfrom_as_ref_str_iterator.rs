// Generated macro for from_as_ref_str_iterator (function)
macro_rules! Depcrate_repr_iterfrom_as_ref_str_iterator {
() => {
// Module: crate::repr::iter
// Provides: {"from_as_ref_str_iterator"}
// Dependencies: {}
fn from_as_ref_str_iterator < S , I > (mut iter : I) -> Repr where S : AsRef < str > , I : Iterator < Item = S > , String : core :: iter :: Extend < S > , String : FromIterator < S > , { let mut curr_len = 0 ; let mut inline_buf = InlineBuffer :: new_const ("") ; while let Some (s) = iter . next () { let str_slice = s . as_ref () ; let bytes_len = str_slice . len () ; if bytes_len + curr_len > MAX_SIZE { let (min_remaining , _) = iter . size_hint () ; let mut string = String :: with_capacity (bytes_len + curr_len + min_remaining) ; string . push_str (unsafe { core :: str :: from_utf8_unchecked (& inline_buf . 0 [.. curr_len]) }) ; string . push_str (str_slice) ; string . extend (iter) ; return Repr :: from_string (string , true) . unwrap_with_msg () ; } inline_buf . 0 [curr_len ..] [.. bytes_len] . copy_from_slice (str_slice . as_bytes ()) ; curr_len += bytes_len ; } unsafe { inline_buf . set_len (curr_len) } Repr :: from_inline (inline_buf) }
};
}
