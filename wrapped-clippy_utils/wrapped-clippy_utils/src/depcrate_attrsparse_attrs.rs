// Generated macro for parse_attrs (function)
macro_rules! Depcrate_attrsparse_attrs {
() => {
// Module: crate::attrs
// Provides: {"parse_attrs"}
// Dependencies: {}
fn parse_attrs < F : FnMut (u64) > (sess : & Session , attrs : & [impl AttributeExt] , name : Symbol , mut f : F) { for attr in get_builtin_attr (sess , attrs , name) { let Some (value) = attr . value_str () else { sess . dcx () . span_err (attr . span () , "bad clippy attribute") ; continue ; } ; let Ok (value) = u64 :: from_str (value . as_str ()) else { sess . dcx () . span_err (attr . span () , "not a number") ; continue ; } ; f (value) ; } }
};
}
