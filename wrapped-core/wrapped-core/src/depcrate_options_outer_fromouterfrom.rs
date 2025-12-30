// Generated macro for OuterFrom (struct)
macro_rules! Depcrate_options_outer_fromOuterFrom {
() => {
// Module: crate::options::outer_from
// Provides: {"OuterFrom"}
// Dependencies: {}
# [doc = " Reusable base for `FromDeriveInput`, `FromVariant`, `FromField`, and other top-level"] # [doc = " `From*` traits."] # [derive (Debug , Clone)] pub struct OuterFrom { # [doc = " The field on the target struct which should receive the type identifier, if any."] pub ident : Option < Ident > , # [doc = " The field on the target struct which should receive the type attributes, if any."] pub attrs : Option < ForwardedField > , pub container : Core , # [doc = " The attribute names that should be searched."] pub attr_names : PathList , # [doc = " The attribute names that should be forwarded. The presence of the word with no additional"] # [doc = " filtering will cause _all_ attributes to be cloned and exposed to the struct after parsing."] pub forward_attrs : Option < ForwardAttrsFilter > , # [doc = " Whether or not the container can be made through conversion from the type `Ident`."] pub from_ident : bool , }
};
}
