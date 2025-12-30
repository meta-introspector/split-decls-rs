// Generated macro for impl_334 (impl)
macro_rules! Depcrate_text_editimpl_334 {
() => {
// Module: crate::text_edit
// Provides: {"impl_334"}
// Dependencies: {}
impl Indel { pub fn insert (offset : TextSize , text : String) -> Indel { Indel :: replace (TextRange :: empty (offset) , text) } pub fn delete (range : TextRange) -> Indel { Indel :: replace (range , String :: new ()) } pub fn replace (range : TextRange , replace_with : String) -> Indel { Indel { delete : range , insert : replace_with } } pub fn apply (& self , text : & mut String) { let start : usize = self . delete . start () . into () ; let end : usize = self . delete . end () . into () ; text . replace_range (start .. end , & self . insert) ; } }
};
}
