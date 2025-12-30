// Generated macro for var_error (module)
macro_rules! Depcrate_arbitrary__std_envvar_error {
() => {
// Module: crate::arbitrary::_std::env
// Provides: {"var_error"}
// Dependencies: {}
# [cfg (not (target_arch = "wasm32"))] mod var_error { use super :: * ; # [doc = " Generates the set of `WTF-16 \\ UTF-16` and makes"] # [doc = " an `OsString` that is not a valid String from it."] # [cfg (target_os = "windows")] fn osstring_invalid_string () -> impl Strategy < Value = OsString > { use std :: os :: windows :: ffi :: OsStringExt ; let size = 1 .. :: std :: u16 :: MAX as usize ; let vec_gen = crate :: collection :: vec (.. :: std :: u16 :: MAX , size . clone ()) ; (size , vec_gen) . prop_map (| (p , mut sbuf) | { let p = :: std :: cmp :: min (p , sbuf . len () - 1) ; make_utf16_invalid (& mut sbuf , p) ; OsString :: from_wide (sbuf . as_slice ()) . into_string () . unwrap_err () }) } # [cfg (not (target_os = "windows"))] fn osstring_invalid_string () -> impl Strategy < Value = OsString > { use crate :: arbitrary :: _std :: string :: not_utf8_bytes ; use std :: os :: unix :: ffi :: OsStringExt ; static_map (not_utf8_bytes (true) , OsString :: from_vec) } arbitrary ! (VarError , TupleUnion < (WA < Just < Self >>, WA < SFnPtrMap < BoxedStrategy < OsString >, Self >>) >; prop_oneof ! [Just (VarError :: NotPresent) , static_map (osstring_invalid_string () . boxed () , VarError :: NotUnicode)]) ; }
};
}
