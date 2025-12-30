// Generated macro for Name (enum)
macro_rules! Depcrate_astName {
() => {
// Module: crate::ast
// Provides: {"Name"}
// Dependencies: {}
# [doc = " The `<name>` production."] # [doc = ""] # [doc = " ```text"] # [doc = " <name> ::= <nested-name>"] # [doc = "        ::= <unscoped-name>"] # [doc = "        ::= <unscoped-template-name> <template-args>"] # [doc = "        ::= <local-name>"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq , Eq)] pub enum Name { # [doc = " A nested name"] Nested (NestedName) , # [doc = " An unscoped name."] Unscoped (UnscopedName) , # [doc = " An unscoped template."] UnscopedTemplate (UnscopedTemplateNameHandle , TemplateArgs) , # [doc = " A local name."] Local (LocalName) , }
};
}
