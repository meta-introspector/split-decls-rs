// Generated macro for UnnamedTypeName (struct)
macro_rules! Depcrate_astUnnamedTypeName {
() => {
// Module: crate::ast
// Provides: {"UnnamedTypeName"}
// Dependencies: {}
# [doc = " The `<unnamed-type-name>` production."] # [doc = ""] # [doc = " ```text"] # [doc = " <unnamed-type-name> ::= Ut [ <nonnegative number> ] _"] # [doc = "                     ::= <closure-type-name>"] # [doc = "     ```"] # [doc = ""] # [doc = " We handle `<closure-type-name>` separately."] # [derive (Clone , Debug , PartialEq , Eq)] pub struct UnnamedTypeName (Option < usize >) ;
};
}
