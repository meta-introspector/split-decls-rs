// Generated macro for NestedName (enum)
macro_rules! Depcrate_astNestedName {
() => {
// Module: crate::ast
// Provides: {"NestedName"}
// Dependencies: {}
# [doc = " The `<nested-name>` production."] # [doc = ""] # [doc = " ```text"] # [doc = " <nested-name> ::= N [<CV-qualifiers>] [<ref-qualifier>] <prefix> <unqualified-name> E"] # [doc = "               ::= N [<CV-qualifiers>] [<ref-qualifier>] <template-prefix> <template-args> E"] # [doc = "               ::= N H <prefix> <unqualified-name> E"] # [doc = "               ::= N H <template-prefix> <template-args> E"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq , Eq)] pub enum NestedName { # [doc = " A nested name."] Unqualified (CvQualifiers , Option < RefQualifier > , Option < PrefixHandle > , UnqualifiedName ,) , # [doc = " A nested template name. The `<template-args>` are part of the `PrefixHandle`."] Template (CvQualifiers , Option < RefQualifier > , PrefixHandle) , # [doc = " A nested name with an explicit object."] UnqualifiedExplicitObject (Option < PrefixHandle > , UnqualifiedName , ExplicitObjectParameter ,) , # [doc = " A nested template name with an explicit object."] TemplateExplicitObject (PrefixHandle , ExplicitObjectParameter) , }
};
}
