// Generated macro for impl_302 (impl)
macro_rules! Depcrate_text_editimpl_302 {
() => {
// Module: crate::text_edit
// Provides: {"impl_302"}
// Dependencies: {}
impl TextEditBuilder { pub fn is_empty (& self) -> bool { self . indels . is_empty () } pub fn replace (& mut self , range : TextRange , replace_with : String) { self . indel (Indel :: replace (range , replace_with)) ; } pub fn delete (& mut self , range : TextRange) { self . indel (Indel :: delete (range)) ; } pub fn insert (& mut self , offset : TextSize , text : String) { self . indel (Indel :: insert (offset , text)) ; } pub fn finish (self) -> TextEdit { let TextEditBuilder { mut indels , annotation } = self ; assert_disjoint_or_equal (& mut indels) ; indels = coalesce_indels (indels) ; TextEdit { indels , annotation } } pub fn invalidates_offset (& self , offset : TextSize) -> bool { self . indels . iter () . any (| indel | indel . delete . contains_inclusive (offset)) } pub fn indel (& mut self , indel : Indel) { self . indels . push (indel) ; if self . indels . len () <= 16 { assert_disjoint_or_equal (& mut self . indels) ; } } }
};
}
