// Generated macro for ClassEnumType (enum)
macro_rules! Depcrate_astClassEnumType {
() => {
// Module: crate::ast
// Provides: {"ClassEnumType"}
// Dependencies: {}
# [doc = " The `<class-enum-type>` production."] # [doc = ""] # [doc = " ```text"] # [doc = " <class-enum-type> ::= <name>"] # [doc = "                   ::= Ts <name>"] # [doc = "                   ::= Tu <name>"] # [doc = "                   ::= Te <name>"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq , Eq)] pub enum ClassEnumType { # [doc = " A non-dependent type name, dependent type name, or dependent"] # [doc = " typename-specifier."] Named (Name) , # [doc = " A dependent elaborated type specifier using 'struct' or 'class'."] ElaboratedStruct (Name) , # [doc = " A dependent elaborated type specifier using 'union'."] ElaboratedUnion (Name) , # [doc = " A dependent elaborated type specifier using 'enum'."] ElaboratedEnum (Name) , }
};
}
