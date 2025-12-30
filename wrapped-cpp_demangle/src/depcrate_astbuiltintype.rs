// Generated macro for BuiltinType (enum)
macro_rules! Depcrate_astBuiltinType {
() => {
// Module: crate::ast
// Provides: {"BuiltinType"}
// Dependencies: {}
# [doc = " The `<builtin-type>` production."] # [derive (Clone , Debug , PartialEq , Eq)] pub enum BuiltinType { # [doc = " A simple standards compliant builtin type."] Standard (StandardBuiltinType) , # [doc = " A standards compliant builtin type with a parameter, e.g. _BitInt(32)."] Parametric (ParametricBuiltinType) , # [doc = " A non-standard, vendor extension type."] # [doc = ""] # [doc = " ```text"] # [doc = " <builtin-type> ::= u <source-name>   # vendor extended type"] # [doc = " ```"] Extension (SourceName) , }
};
}
