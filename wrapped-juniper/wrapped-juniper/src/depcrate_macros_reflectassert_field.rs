// Generated macro for assert_field (macro)
macro_rules! Depcrate_macros_reflectassert_field {
() => {
// Module: crate::macros::reflect
// Provides: {"assert_field"}
// Dependencies: {}
# [doc = " Asserts validness of [`Field`] [`Arguments`] and returned [`Type`]."] # [doc = ""] # [doc = " This assertion is a combination of [`assert_subtype`] and"] # [doc = " [`assert_field_args`]."] # [doc = ""] # [doc = " See [spec][1] for more info."] # [doc = ""] # [doc = " [1]: https://spec.graphql.org/October2021#IsValidImplementation()"] # [macro_export] macro_rules ! assert_field { ($ base_ty : ty , $ impl_ty : ty , $ scalar : ty , $ field_name : expr $ (,) ?) => { $ crate :: assert_field_args ! ($ base_ty , $ impl_ty , $ scalar , $ field_name) ; $ crate :: assert_subtype ! ($ base_ty , $ impl_ty , $ scalar , $ field_name) ; } ; }
};
}
