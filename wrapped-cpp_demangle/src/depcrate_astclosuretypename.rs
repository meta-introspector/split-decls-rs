// Generated macro for ClosureTypeName (struct)
macro_rules! Depcrate_astClosureTypeName {
() => {
// Module: crate::ast
// Provides: {"ClosureTypeName"}
// Dependencies: {}
# [doc = " The `<closure-type-name>` production."] # [doc = ""] # [doc = " ```text"] # [doc = " <closure-type-name> ::= Ul <lambda-sig> E [ <nonnegative number> ] _"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq , Eq)] pub struct ClosureTypeName (LambdaSig , Option < usize >) ;
};
}
