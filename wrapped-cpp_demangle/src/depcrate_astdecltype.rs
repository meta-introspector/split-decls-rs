// Generated macro for Decltype (enum)
macro_rules! Depcrate_astDecltype {
() => {
// Module: crate::ast
// Provides: {"Decltype"}
// Dependencies: {}
# [doc = " The `<decltype>` production."] # [doc = ""] # [doc = " ```text"] # [doc = " <decltype> ::= Dt <expression> E"] # [doc = "            ::= DT <expression> E"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq , Eq)] pub enum Decltype { # [doc = " A `decltype` of an id-expression or class member access (C++0x)."] IdExpression (Expression) , # [doc = " A `decltype` of an expression (C++0x)."] Expression (Expression) , }
};
}
