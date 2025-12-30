// Generated macro for impl_20 (impl)
macro_rules! Depcrateimpl_20 {
() => {
// Module: crate
// Provides: {"impl_20"}
// Dependencies: {}
impl < 'a > Iterator for AsciiCharsIter < 'a > { type Item = Option < & 'static str > ; # [inline] fn next (& mut self) -> Option < Self :: Item > { let dch = self . next_char ? ; self . next_char = self . chars . next () . map (deunicode_char) ; let dch = match dch { None => return Some (None) , Some (dch) => dch , } ; let trim_last_char = dch . as_bytes () . len () > 1 && dch . as_bytes () . last () . copied () == Some (b' ') && self . next_char . map_or (true , | ch | { ch . map_or (false , | ch | ch . as_bytes () . first () . copied () == Some (b' ')) }) ; Some (if ! trim_last_char { Some (dch) } else { dch . get (.. dch . len () - 1) }) } # [inline] fn count (self) -> usize { self . chars . count () + if self . next_char . is_some () { 1 } else { 0 } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { (self . chars . size_hint () . 0 + if self . next_char . is_some () { 1 } else { 0 } , None) } }
};
}
