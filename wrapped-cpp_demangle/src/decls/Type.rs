macro_rules! deps {
    () => {
        TemplateArgs!();
        PointerToMemberType!();
        SourceName!();
        TemplateParam!();
        CvQualifiers!();
        Decltype!();
        FunctionType!();
        ArrayType!();
        VectorType!();
        ClassEnumType!();
    };
}

macro_rules! Type {
    () => {
        deps!();
        # [doc = " The `<type>` production."] # [doc = ""] # [doc = " ```text"] # [doc = " <type> ::= <builtin-type>"] # [doc = "        ::= <function-type>"] # [doc = "        ::= <class-enum-type>"] # [doc = "        ::= <array-type>"] # [doc = "        ::= <vector-type>"] # [doc = "        ::= <pointer-to-member-type>"] # [doc = "        ::= <template-param>"] # [doc = "        ::= <template-template-param> <template-args>"] # [doc = "        ::= <decltype>"] # [doc = "        ::= <CV-qualifiers> <type>"] # [doc = "        ::= P <type>                                 # pointer-to"] # [doc = "        ::= R <type>                                 # reference-to"] # [doc = "        ::= O <type>                                 # rvalue reference-to (C++0x)"] # [doc = "        ::= C <type>                                 # complex pair (C 2000)"] # [doc = "        ::= G <type>                                 # imaginary (C 2000)"] # [doc = "        ::= U <source-name> [<template-args>] <type> # vendor extended type qualifier"] # [doc = "        ::= Dp <type>                                # pack expansion (C++0x)"] # [doc = "        ::= <substitution>"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq , Eq)] # [allow (clippy :: large_enum_variant)] pub enum Type { # [doc = " A function type."] Function (FunctionType) , # [doc = " A class, union, or enum type."] ClassEnum (ClassEnumType) , # [doc = " An array type."] Array (ArrayType) , # [doc = " A vector type."] Vector (VectorType) , # [doc = " A pointer-to-member type."] PointerToMember (PointerToMemberType) , # [doc = " A named template parameter type."] TemplateParam (TemplateParam) , # [doc = " A template template type."] TemplateTemplate (TemplateTemplateParamHandle , TemplateArgs) , # [doc = " A decltype."] Decltype (Decltype) , # [doc = " A const-, restrict-, and/or volatile-qualified type."] Qualified (CvQualifiers , TypeHandle) , # [doc = " A pointer to a type."] PointerTo (TypeHandle) , # [doc = " An lvalue reference to a type."] LvalueRef (TypeHandle) , # [doc = " An rvalue reference to a type."] RvalueRef (TypeHandle) , # [doc = " A complex pair of the given type."] Complex (TypeHandle) , # [doc = " An imaginary of the given type."] Imaginary (TypeHandle) , # [doc = " A vendor extended type qualifier."] VendorExtension (SourceName , Option < TemplateArgs > , TypeHandle) , # [doc = " A pack expansion."] PackExpansion (TypeHandle) , }
    };
}

Type!();