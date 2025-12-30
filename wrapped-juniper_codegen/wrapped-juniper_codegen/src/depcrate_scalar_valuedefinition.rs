// Generated macro for Definition (struct)
macro_rules! Depcrate_scalar_valueDefinition {
() => {
// Module: crate::scalar_value
// Provides: {"Definition"}
// Dependencies: {}
# [doc = " Definition of a `ScalarValue` for code generation."] struct Definition { # [doc = " [`syn::Ident`] of the enum representing this `ScalarValue`."] ident : syn :: Ident , # [doc = " [`syn::Generics`] of the enum representing this `ScalarValue`."] generics : syn :: Generics , # [doc = " [`syn::Variant`]s of the enum representing this `ScalarValue`."] variants : Vec < syn :: Variant > , # [doc = " [`Variant`]s marked with a [`Method`] attribute."] methods : HashMap < Method , Vec < Variant > > , # [doc = " Custom definition to call in `ScalarValue::from_displayable()` method."] # [doc = ""] # [doc = " If [`None`] then `ScalarValue::from_displayable()` method is not generated."] from_displayable : Option < syn :: ExprPath > , # [doc = " Custom definition to call in `ScalarValue::from_displayable_non_static()` method."] # [doc = ""] # [doc = " If [`None`] then `ScalarValue::from_displayable_non_static()` method is not generated."] from_displayable_non_static : Option < syn :: ExprPath > , }
};
}
