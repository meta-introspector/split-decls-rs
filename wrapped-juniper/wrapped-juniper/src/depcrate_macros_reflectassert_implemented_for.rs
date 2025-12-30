// Generated macro for assert_implemented_for (macro)
macro_rules! Depcrate_macros_reflectassert_implemented_for {
() => {
// Module: crate::macros::reflect
// Provides: {"assert_implemented_for"}
// Dependencies: {}
# [doc = " Asserts that `#[graphql_interface(for = ...)]` has all the types referencing"] # [doc = " this interface in the `impl = ...` attribute argument."] # [doc = ""] # [doc = " Symmetrical to [`assert_interfaces_impls!`]."] # [macro_export] macro_rules ! assert_implemented_for { ($ scalar : ty , $ implementor : ty $ (, $ interfaces : ty) * $ (,) ?) => { const _ : () = { $ ({ let is_present = $ crate :: macros :: reflect :: str_exists_in_arr (<$ implementor as :: juniper :: macros :: reflect :: BaseType <$ scalar >>:: NAME , <$ interfaces as :: juniper :: macros :: reflect :: BaseSubTypes <$ scalar >>:: NAMES ,) ; if ! is_present { const MSG : & str = $ crate :: const_concat ! ("Failed to implement interface `" , <$ interfaces as $ crate :: macros :: reflect :: BaseType <$ scalar >>:: NAME , "` on `" , <$ implementor as $ crate :: macros :: reflect :: BaseType <$ scalar >>:: NAME , "`: missing implementer reference in interface's `for` attribute." ,) ; :: core :: panic ! ("{}" , MSG) ; } }) * } ; } ; }
};
}
