// Generated macro for FdiOptions (struct)
macro_rules! Depcrate_options_from_deriveFdiOptions {
() => {
// Module: crate::options::from_derive
// Provides: {"FdiOptions"}
// Dependencies: {}
# [derive (Debug)] pub struct FdiOptions { pub base : OuterFrom , # [doc = " The field on the target struct which should receive the type visibility, if any."] pub vis : Option < Ident > , # [doc = " The field on the target struct which should receive the type generics, if any."] pub generics : Option < ForwardedField > , # [doc = " The field on the target struct which should receive the derive input body, if any."] pub data : Option < ForwardedField > , pub supports : Option < DeriveInputShapeSet > , }
};
}
