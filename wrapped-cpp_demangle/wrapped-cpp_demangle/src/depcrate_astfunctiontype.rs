// Generated macro for FunctionType (struct)
macro_rules! Depcrate_astFunctionType {
() => {
// Module: crate::ast
// Provides: {"FunctionType"}
// Dependencies: {}
# [doc = " The `<function-type>` production."] # [doc = ""] # [doc = " ```text"] # [doc = " <function-type> ::= [<CV-qualifiers>] [exception-spec] [Dx] F [Y] <bare-function-type> [<ref-qualifier>] E"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq , Eq)] pub struct FunctionType { cv_qualifiers : CvQualifiers , exception_spec : Option < ExceptionSpec > , transaction_safe : bool , extern_c : bool , bare : BareFunctionType , ref_qualifier : Option < RefQualifier > , }
};
}
