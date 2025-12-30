// Generated macro for XmlWriter (struct)
macro_rules! Depcrate_stream_xml_writerXmlWriter {
() => {
// Module: crate::stream::xml_writer
// Provides: {"XmlWriter"}
// Dependencies: {}
pub struct XmlWriter < W : Write > { xml_writer : EventWriter < W > , write_root_element : bool , indent_char : u8 , indent_count : usize , started_plist : bool , stack : Vec < Element > , expecting_key : bool , pending_collection : Option < PendingCollection > , }
};
}
