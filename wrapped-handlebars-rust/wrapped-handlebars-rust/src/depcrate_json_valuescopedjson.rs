// Generated macro for ScopedJson (enum)
macro_rules! Depcrate_json_valueScopedJson {
() => {
// Module: crate::json::value
// Provides: {"ScopedJson"}
// Dependencies: {}
# [doc = " A JSON wrapper designed for handlebars internal use case"] # [doc = ""] # [doc = " * Constant: the JSON value hardcoded into template"] # [doc = " * Context:  the JSON value referenced in your provided data context"] # [doc = " * Derived:  the owned JSON value computed during rendering process"] # [doc = ""] # [derive (Debug , Clone)] pub enum ScopedJson < 'rc > { Constant (& 'rc Json) , Derived (Json) , Context (& 'rc Json , Vec < String >) , Missing , }
};
}
