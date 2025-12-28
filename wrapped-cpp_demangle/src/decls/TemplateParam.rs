macro_rules! TemplateParam {
    () => {
        # [doc = " The `<template-param>` production."] # [doc = ""] # [doc = " ```text"] # [doc = " <template-param> ::= T_ # first template parameter"] # [doc = "                  ::= T <parameter-2 non-negative number> _"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq , Eq)] pub struct TemplateParam (usize) ;
    };
}

TemplateParam!()