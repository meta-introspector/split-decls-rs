// Generated macro for InputValue (enum)
macro_rules! Depcrate_astInputValue {
() => {
// Module: crate::ast
// Provides: {"InputValue"}
// Dependencies: {}
# [doc = " A JSON-like value that can be passed into the query execution, either"] # [doc = " out-of-band, or in-band as default variable values. These are _not_ constant"] # [doc = " and might contain variables."] # [doc = ""] # [doc = " Lists and objects variants are _spanned_, i.e. they contain a reference to"] # [doc = " their position in the source file, if available."] # [expect (missing_docs , reason = "self-explanatory")] # [derive (Clone , Debug , PartialEq)] pub enum InputValue < S = DefaultScalarValue > { Null , Scalar (S) , Enum (String) , Variable (String) , List (Vec < Spanning < InputValue < S > > >) , Object (Vec < (Spanning < String > , Spanning < InputValue < S > >) >) , }
};
}
