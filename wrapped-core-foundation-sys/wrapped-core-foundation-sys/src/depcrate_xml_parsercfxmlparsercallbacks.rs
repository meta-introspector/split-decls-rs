// Generated macro for CFXMLParserCallBacks (struct)
macro_rules! Depcrate_xml_parserCFXMLParserCallBacks {
() => {
// Module: crate::xml_parser
// Provides: {"CFXMLParserCallBacks"}
// Dependencies: {}
# [repr (C)] # [derive (Debug , Clone , Copy)] pub struct CFXMLParserCallBacks { pub version : CFIndex , pub createXMLStructure : CFXMLParserCreateXMLStructureCallBack , pub addChild : CFXMLParserAddChildCallBack , pub endXMLStructure : CFXMLParserEndXMLStructureCallBack , pub resolveExternalEntity : CFXMLParserResolveExternalEntityCallBack , pub handleError : CFXMLParserHandleErrorCallBack , }
};
}
