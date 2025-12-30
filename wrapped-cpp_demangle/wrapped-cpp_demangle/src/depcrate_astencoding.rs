// Generated macro for Encoding (enum)
macro_rules! Depcrate_astEncoding {
() => {
// Module: crate::ast
// Provides: {"Encoding"}
// Dependencies: {}
# [doc = " The `<encoding>` production."] # [doc = ""] # [doc = " ```text"] # [doc = " <encoding> ::= <function name> <bare-function-type>"] # [doc = "            ::= <data name>"] # [doc = "            ::= <special-name>"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq , Eq)] pub enum Encoding { # [doc = " An encoded function."] Function (Name , BareFunctionType) , # [doc = " An encoded static variable."] Data (Name) , # [doc = " A special encoding."] Special (SpecialName) , }
};
}
