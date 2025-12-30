// Generated macro for assert_transitive_impls (macro)
macro_rules! Depcrate_macros_reflectassert_transitive_impls {
() => {
// Module: crate::macros::reflect
// Provides: {"assert_transitive_impls"}
// Dependencies: {}
# [doc = " Asserts that all [transitive interfaces][0] (the ones implemented by the"] # [doc = " `$interface`) are also implemented by the `$implementor`."] # [doc = ""] # [doc = " [0]: https://spec.graphql.org/October2021#sel-FAHbhBHCAACGB35P"] # [macro_export] macro_rules ! assert_transitive_impls { ($ scalar : ty , $ interface : ty , $ implementor : ty $ (, $ transitive : ty) * $ (,) ?) => { const _ : () = { $ ({ let is_present = $ crate :: macros :: reflect :: str_exists_in_arr (<$ implementor as :: juniper :: macros :: reflect :: BaseType <$ scalar >>:: NAME , <$ transitive as :: juniper :: macros :: reflect :: BaseSubTypes <$ scalar >>:: NAMES ,) ; if ! is_present { const MSG : & str = $ crate :: const_concat ! ("Failed to implement interface `" , <$ interface as $ crate :: macros :: reflect :: BaseType <$ scalar >>:: NAME , "` on `" , <$ implementor as $ crate :: macros :: reflect :: BaseType <$ scalar >>:: NAME , "`: missing `impl = ` for transitive interface `" , <$ transitive as $ crate :: macros :: reflect :: BaseType <$ scalar >>:: NAME , "` on `" , <$ implementor as $ crate :: macros :: reflect :: BaseType <$ scalar >>:: NAME , "`.") ; :: core :: panic ! ("{}" , MSG) ; } }) * } ; } ; }
};
}
