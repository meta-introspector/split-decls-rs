// Generated macro for FromFieldImpl (struct)
macro_rules! Depcrate_codegen_from_fieldFromFieldImpl {
() => {
// Module: crate::codegen::from_field
// Provides: {"FromFieldImpl"}
// Dependencies: {}
# [doc = " `impl FromField` generator. This is used for parsing an individual"] # [doc = " field and its attributes."] pub struct FromFieldImpl < 'a > { pub ident : Option < & 'a Ident > , pub vis : Option < & 'a Ident > , pub ty : Option < & 'a Ident > , pub base : TraitImpl < 'a > , pub attr_names : & 'a PathList , pub forward_attrs : ForwardAttrs < 'a > , pub from_ident : bool , }
};
}
