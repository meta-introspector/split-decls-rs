// Generated macro for impl_77 (impl)
macro_rules! Depcrate_shared_util_array_strimpl_77 {
() => {
// Module: crate::shared::util::array_str
// Provides: {"impl_77"}
// Dependencies: {}
impl < const N : usize > ArrayStr < N > { # [doc = " Creates a new fixed capacity string."] # [doc = ""] # [doc = " If the given string exceeds `N` bytes, then this returns"] # [doc = " `None`."] pub (crate) const fn new (s : & str) -> Option < ArrayStr < N > > { let len = s . len () ; if len > N { return None ; } let mut bytes = [0 ; N] ; let mut i = 0 ; while i < s . as_bytes () . len () { bytes [i] = s . as_bytes () [i] ; i += 1 ; } debug_assert ! (N <= u8 :: MAX as usize , "size of ArrayStr is too big") ; Some (ArrayStr { bytes , len : len as u8 }) } # [doc = " Returns the capacity of this array string."] pub (crate) fn capacity () -> usize { N } # [doc = " Append the bytes given to the end of this string."] # [doc = ""] # [doc = " If the capacity would be exceeded, then this is a no-op and `false`"] # [doc = " is returned."] pub (crate) fn push_str (& mut self , s : & str) -> bool { let len = usize :: from (self . len) ; let Some (new_len) = len . checked_add (s . len ()) else { return false } ; if new_len > N { return false ; } self . bytes [len .. new_len] . copy_from_slice (s . as_bytes ()) ; debug_assert ! (N <= usize :: from (u8 :: MAX) , "size of ArrayStr is too big") ; self . len = u8 :: try_from (new_len) . unwrap () ; true } # [doc = " Returns this array string as a string slice."] pub (crate) fn as_str (& self) -> & str { core :: str :: from_utf8 (& self . bytes [.. usize :: from (self . len)]) . unwrap () } }
};
}
