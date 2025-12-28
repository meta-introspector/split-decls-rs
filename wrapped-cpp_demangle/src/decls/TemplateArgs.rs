macro_rules! deps {
    () => {
        TemplateArg!();
    };
}

macro_rules! TemplateArgs {
    () => {
        deps!();
        # [doc = " The `<template-args>` production."] # [doc = ""] # [doc = " ```text"] # [doc = " <template-args> ::= I <template-arg>+ E"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq , Eq)] pub struct TemplateArgs (Vec < TemplateArg >) ;
    };
}

TemplateArgs!()