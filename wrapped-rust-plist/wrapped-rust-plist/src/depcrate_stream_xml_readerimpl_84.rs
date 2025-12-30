// Generated macro for impl_84 (impl)
macro_rules! Depcrate_stream_xml_readerimpl_84 {
() => {
// Module: crate::stream::xml_reader
// Provides: {"impl_84"}
// Dependencies: {}
impl < R : BufRead > XmlReader < R > { pub fn new (reader : R) -> XmlReader < R > { let mut xml_reader = EventReader :: from_reader (reader) ; let config = xml_reader . config_mut () ; config . trim_text (false) ; config . check_end_names = true ; config . expand_empty_elements = true ; XmlReader { buffer : Vec :: new () , started : false , finished : false , state : ReaderState (xml_reader) , } } pub fn into_inner (self) -> R { self . state . 0 . into_inner () } pub (crate) fn xml_doc_started (& self) -> bool { self . started } }
};
}
