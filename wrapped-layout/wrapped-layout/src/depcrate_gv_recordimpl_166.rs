// Generated macro for impl_166 (impl)
macro_rules! Depcrate_gv_recordimpl_166 {
() => {
// Module: crate::gv::record
// Provides: {"impl_166"}
// Dependencies: {}
impl RecordParserFrame { pub fn new () -> Self { Self { label : String :: new () , arr : Vec :: new () , } } # [doc = " Split a label such as \"<f0> XXX\" into the port part \"f0\" and the text"] # [doc = " part \"XXX\"."] fn split_label_to_text_and_port (str : & str) -> (String , Option < String >) { let str = str . trim () ; if str . starts_with ('<') { if let Option :: Some (idx) = str . find ('>') { let port = & str [1 .. idx] ; return (str [idx + 1 ..] . trim () . to_string () , Option :: Some (port . to_string ()) ,) ; } } (str . to_string () , Option :: None) } pub fn finalize_label (& mut self) { if ! self . label . trim () . is_empty () { let ret = Self :: split_label_to_text_and_port (& self . label) ; let text = RecordDef :: Text (ret . 0 , ret . 1) ; self . arr . push (text) ; self . label . clear () ; } } pub fn finalize_record (& mut self) -> RecordDef { self . finalize_label () ; match self . arr . len () { 0 => RecordDef :: Text (String :: from ("") , Option :: None) , _ => RecordDef :: Array (self . arr . clone ()) , } } }
};
}
