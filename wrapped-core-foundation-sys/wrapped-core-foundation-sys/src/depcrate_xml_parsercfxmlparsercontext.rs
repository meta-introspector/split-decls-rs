// Generated macro for CFXMLParserContext (struct)
macro_rules! Depcrate_xml_parserCFXMLParserContext {
() => {
// Module: crate::xml_parser
// Provides: {"CFXMLParserContext"}
// Dependencies: {}
# [repr (C)] # [derive (Debug , Clone , Copy)] pub struct CFXMLParserContext { pub version : CFIndex , pub info : * mut c_void , pub retain : CFXMLParserRetainCallBack , pub release : CFXMLParserReleaseCallBack , pub copyDescription : CFXMLParserCopyDescriptionCallBack , }
};
}
