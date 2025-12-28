macro_rules! deps {
    () => {
        UnscopedTemplateName!();
        TemplateTemplateParam!();
        Type!();
        UnresolvedType!();
        Prefix!();
    };
}

macro_rules! Substitutable {
    () => {
        deps!();
        # [doc = " An enumeration of all of the types that can end up in the substitution"] # [doc = " table."] # [doc (hidden)] # [derive (Clone , Debug , PartialEq , Eq)] # [allow (clippy :: large_enum_variant)] pub enum Substitutable { # [doc = " An `<unscoped-template-name>` production."] UnscopedTemplateName (ast :: UnscopedTemplateName) , # [doc = " A `<type>` production."] Type (ast :: Type) , # [doc = " A `<template-template-param>` production."] TemplateTemplateParam (ast :: TemplateTemplateParam) , # [doc = " An `<unresolved-type>` production."] UnresolvedType (ast :: UnresolvedType) , # [doc = " A `<prefix>` production."] Prefix (ast :: Prefix) , }
    };
}

Substitutable!()