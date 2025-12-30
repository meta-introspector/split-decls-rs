// Generated macro for assert_interfaces_impls (macro)
macro_rules! Depcrate_macros_reflectassert_interfaces_impls {
() => {
// Module: crate::macros::reflect
// Provides: {"assert_interfaces_impls"}
// Dependencies: {}
# [doc = " Asserts that `impl = ...` attribute argument has all the types referencing"] # [doc = " this GraphQL type in `#[graphql_interface(for = ...)]`."] # [doc = ""] # [doc = " Symmetrical to [`assert_implemented_for!`]."] # [macro_export] macro_rules ! assert_interfaces_impls { ($ scalar : ty , $ interface : ty $ (, $ implementers : ty) * $ (,) ?) => { const _ : () = { $ ({ let is_present = $ crate :: macros :: reflect :: str_exists_in_arr (<$ interface as :: juniper :: macros :: reflect :: BaseType <$ scalar >>:: NAME , <$ implementers as :: juniper :: macros :: reflect :: Implements <$ scalar >>:: NAMES ,) ; if ! is_present { const MSG : & str = $ crate :: const_concat ! ("Failed to implement interface `" , <$ interface as $ crate :: macros :: reflect :: BaseType <$ scalar >>:: NAME , "` on `" , <$ implementers as $ crate :: macros :: reflect :: BaseType <$ scalar >>:: NAME , "`: missing interface reference in implementer's `impl` attribute." ,) ; :: core :: panic ! ("{}" , MSG) ; } }) * } ; } ; }
};
}
