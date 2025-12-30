// Generated macro for CFXMLParserCreateXMLStructureCallBack (type)
macro_rules! Depcrate_xml_parserCFXMLParserCreateXMLStructureCallBack {
() => {
// Module: crate::xml_parser
// Provides: {"CFXMLParserCreateXMLStructureCallBack"}
// Dependencies: {}
pub type CFXMLParserCreateXMLStructureCallBack = extern "C" fn (parser : CFXMLParserRef , nodeDesc : CFXMLNodeRef , info : * mut c_void) -> * mut c_void ;
};
}
