// Generated macro for impl_19 (impl)
macro_rules! Depcrateimpl_19 {
() => {
// Module: crate
// Provides: {"impl_19"}
// Dependencies: {}
impl < 'a > Iterator for ByteSerialize < 'a > { type Item = & 'a str ; fn next (& mut self) -> Option < & 'a str > { if let Some ((& first , tail)) = self . bytes . split_first () { if ! byte_serialized_unchanged (first) { self . bytes = tail ; return Some (if first == b' ' { "+" } else { percent_encode_byte (first) }) ; } let position = tail . iter () . position (| & b | ! byte_serialized_unchanged (b)) ; let (unchanged_slice , remaining) = match position { Some (i) => self . bytes . split_at (1 + i) , None => (self . bytes , & [] [..]) , } ; self . bytes = remaining ; Some (unsafe { str :: from_utf8_unchecked (unchanged_slice) }) } else { None } } fn size_hint (& self) -> (usize , Option < usize >) { if self . bytes . is_empty () { (0 , Some (0)) } else { (1 , Some (self . bytes . len ())) } } }
};
}
