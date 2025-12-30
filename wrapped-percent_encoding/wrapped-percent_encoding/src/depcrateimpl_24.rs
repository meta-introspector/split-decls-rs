// Generated macro for impl_24 (impl)
macro_rules! Depcrateimpl_24 {
() => {
// Module: crate
// Provides: {"impl_24"}
// Dependencies: {}
impl < 'a > Iterator for PercentEncode < 'a > { type Item = & 'a str ; fn next (& mut self) -> Option < & 'a str > { if let Some ((& first_byte , remaining)) = self . bytes . split_first () { if self . ascii_set . should_percent_encode (first_byte) { self . bytes = remaining ; Some (percent_encode_byte (first_byte)) } else { for (i , & byte) in remaining . iter () . enumerate () { if self . ascii_set . should_percent_encode (byte) { let (unchanged_slice , remaining) = self . bytes . split_at (1 + i) ; self . bytes = remaining ; return Some (unsafe { str :: from_utf8_unchecked (unchanged_slice) }) ; } } let unchanged_slice = self . bytes ; self . bytes = & [] [..] ; Some (unsafe { str :: from_utf8_unchecked (unchanged_slice) }) } } else { None } } fn size_hint (& self) -> (usize , Option < usize >) { if self . bytes . is_empty () { (0 , Some (0)) } else { (1 , Some (self . bytes . len ())) } } }
};
}
