// Generated macro for Field (struct)
macro_rules! Depcrate_codegen_fieldField {
() => {
// Module: crate::codegen::field
// Provides: {"Field"}
// Dependencies: {}
# [doc = " Properties needed to generate code for a field in all the contexts"] # [doc = " where one may appear."] # [derive (Debug , Clone)] pub struct Field < 'a > { # [doc = " The name presented to the user of the library. This will appear"] # [doc = " in error messages and will be looked when parsing names."] pub name_in_attr : Cow < 'a , str > , # [doc = " The name presented to the author of the library. This will appear"] # [doc = " in the setters or temporary variables which contain the values."] pub ident : & 'a Ident , # [doc = " The type of the field in the input."] pub ty : & 'a Type , pub default_expression : Option < DefaultExpression < 'a > > , # [doc = " An expression that will be wrapped in a call to [`core::convert::identity`] and"] # [doc = " then used for converting a provided value into the field value _before_ postfix"] # [doc = " transforms are called."] pub with_callable : Cow < 'a , syn :: Expr > , pub post_transform : Option < & 'a PostfixTransform > , pub skip : bool , pub multiple : bool , # [doc = " If set, this field will be given all unclaimed meta items and will"] # [doc = " not be exposed as a standard named field."] pub flatten : bool , }
};
}
