// Generated macro for JsNamespace (struct)
macro_rules! Depcrate_parserJsNamespace {
() => {
// Module: crate::parser
// Provides: {"JsNamespace"}
// Dependencies: {}
# [doc = " A list of identifiers representing the namespace prefix of an imported"] # [doc = " function or constant, or for exported types."] # [doc = ""] # [doc = " The list is guaranteed to be non-empty and not start with a non-value JS keyword"] # [doc = " (except for \"default\", which is allowed as a special case)."] # [cfg_attr (feature = "extra-traits" , derive (Debug))] # [derive (Clone)] pub struct JsNamespace (Vec < String >) ;
};
}
