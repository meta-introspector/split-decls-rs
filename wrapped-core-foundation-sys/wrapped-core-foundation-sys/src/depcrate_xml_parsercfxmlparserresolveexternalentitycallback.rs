// Generated macro for CFXMLParserResolveExternalEntityCallBack (type)
macro_rules! Depcrate_xml_parserCFXMLParserResolveExternalEntityCallBack {
() => {
// Module: crate::xml_parser
// Provides: {"CFXMLParserResolveExternalEntityCallBack"}
// Dependencies: {}
pub type CFXMLParserResolveExternalEntityCallBack = extern "C" fn (parser : CFXMLParserRef , extID : * mut CFXMLExternalID , info : * mut c_void ,) -> CFDataRef ;
};
}
