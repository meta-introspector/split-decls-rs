// Generated macro for impl_425 (impl)
macro_rules! Depcrate_traitsimpl_425 {
() => {
// Module: crate::traits
// Provides: {"impl_425"}
// Dependencies: {}
impl < 'a , 'b > FindSubstring < & 'b [u8] > for & 'a [u8] { fn find_substring (& self , substr : & 'b [u8]) -> Option < usize > { if substr . len () > self . len () { return None ; } let (& substr_first , substr_rest) = match substr . split_first () { Some (split) => split , None => return Some (0) , } ; if substr_rest . is_empty () { return memchr :: memchr (substr_first , self) ; } let mut offset = 0 ; let haystack = & self [.. self . len () - substr_rest . len ()] ; while let Some (position) = memchr :: memchr (substr_first , & haystack [offset ..]) { offset += position ; let next_offset = offset + 1 ; if & self [next_offset ..] [.. substr_rest . len ()] == substr_rest { return Some (offset) ; } offset = next_offset ; } None } }
};
}
