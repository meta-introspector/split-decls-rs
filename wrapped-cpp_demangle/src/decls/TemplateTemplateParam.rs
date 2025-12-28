macro_rules! deps {
    () => {
        TemplateParam!();
    };
}

macro_rules! TemplateTemplateParam {
    () => {
        deps!();
        # [doc = " The `<template-template-param>` production."] # [doc = ""] # [doc = " ```text"] # [doc = " <template-template-param> ::= <template-param>"] # [doc = "                           ::= <substitution>"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq , Eq)] pub struct TemplateTemplateParam (TemplateParam) ;
    };
}

TemplateTemplateParam!()