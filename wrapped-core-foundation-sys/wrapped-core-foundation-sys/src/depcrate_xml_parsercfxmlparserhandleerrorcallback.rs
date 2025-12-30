// Generated macro for CFXMLParserHandleErrorCallBack (type)
macro_rules! Depcrate_xml_parserCFXMLParserHandleErrorCallBack {
() => {
// Module: crate::xml_parser
// Provides: {"CFXMLParserHandleErrorCallBack"}
// Dependencies: {}
pub type CFXMLParserHandleErrorCallBack = extern "C" fn (parser : CFXMLParserRef , error : CFXMLParserStatusCode , info : * mut c_void ,) -> Boolean ;
};
}
