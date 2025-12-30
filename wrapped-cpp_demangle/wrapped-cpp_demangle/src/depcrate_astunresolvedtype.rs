// Generated macro for UnresolvedType (enum)
macro_rules! Depcrate_astUnresolvedType {
() => {
// Module: crate::ast
// Provides: {"UnresolvedType"}
// Dependencies: {}
# [doc = " The `<unresolved-type>` production."] # [doc = ""] # [doc = " ```text"] # [doc = " <unresolved-type> ::= <template-param> [ <template-args> ]  # T:: or T<X,Y>::"] # [doc = "                   ::= <decltype>                            # decltype(p)::"] # [doc = "                   ::= <substitution>"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq , Eq)] pub enum UnresolvedType { # [doc = " An unresolved template type."] Template (TemplateParam , Option < TemplateArgs >) , # [doc = " An unresolved `decltype`."] Decltype (Decltype) , }
};
}
