// Generated macro for find_header_value (function)
macro_rules! Depcrate_request_stubfind_header_value {
() => {
// Module: crate::request_stub
// Provides: {"find_header_value"}
// Dependencies: {}
pub fn find_header_value (hdrs : & [HttpHeader] , name : & str) -> Option < String > { hdrs . iter () . find (| & h | h . name == name) . map (| h | h . value . clone ()) }
};
}
