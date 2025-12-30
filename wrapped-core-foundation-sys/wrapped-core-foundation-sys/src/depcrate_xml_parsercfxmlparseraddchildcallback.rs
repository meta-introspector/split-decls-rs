// Generated macro for CFXMLParserAddChildCallBack (type)
macro_rules! Depcrate_xml_parserCFXMLParserAddChildCallBack {
() => {
// Module: crate::xml_parser
// Provides: {"CFXMLParserAddChildCallBack"}
// Dependencies: {}
pub type CFXMLParserAddChildCallBack = extern "C" fn (parser : CFXMLParserRef , parent : * mut c_void , child : * mut c_void , info : * mut c_void ,) ;
};
}
