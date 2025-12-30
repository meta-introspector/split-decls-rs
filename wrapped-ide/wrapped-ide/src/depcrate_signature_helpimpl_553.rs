// Generated macro for impl_553 (impl)
macro_rules! Depcrate_signature_helpimpl_553 {
() => {
// Module: crate::signature_help
// Provides: {"impl_553"}
// Dependencies: {}
impl SignatureHelp { pub fn parameter_labels (& self) -> impl Iterator < Item = & str > + '_ { self . parameters . iter () . map (move | & it | & self . signature [it]) } pub fn parameter_ranges (& self) -> & [TextRange] { & self . parameters } fn push_call_param (& mut self , param : & str) { self . push_param ("(" , param) ; } fn push_generic_param (& mut self , param : & str) { self . push_param ("<" , param) ; } fn push_record_field (& mut self , param : & str) { self . push_param ("{ " , param) ; } fn push_param (& mut self , opening_delim : & str , param : & str) { if ! self . signature . ends_with (opening_delim) { self . signature . push_str (", ") ; } let start = TextSize :: of (& self . signature) ; self . signature . push_str (param) ; let end = TextSize :: of (& self . signature) ; self . parameters . push (TextRange :: new (start , end)) } }
};
}
