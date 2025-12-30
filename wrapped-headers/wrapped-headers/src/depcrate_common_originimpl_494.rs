// Generated macro for impl_494 (impl)
macro_rules! Depcrate_common_originimpl_494 {
() => {
// Module: crate::common::origin
// Provides: {"impl_494"}
// Dependencies: {}
impl OriginOrNull { fn try_from_value (value : & HeaderValue) -> Option < Self > { if value == "null" { return Some (OriginOrNull :: Null) ; } let uri = Uri :: try_from (value . as_bytes ()) . ok () ? ; let (scheme , auth) = match uri . into_parts () { uri :: Parts { scheme : Some (scheme) , authority : Some (auth) , path_and_query : None , .. } => (scheme , auth) , uri :: Parts { scheme : Some (ref scheme) , authority : Some (ref auth) , path_and_query : Some (ref p) , .. } if p == "/" => (scheme . clone () , auth . clone ()) , _ => { return None ; } } ; Some (OriginOrNull :: Origin (scheme , auth)) } }
};
}
