// Generated macro for assert_snapshot (macro)
macro_rules! Depcrate_macrosassert_snapshot {
() => {
// Module: crate::macros
// Provides: {"assert_snapshot"}
// Dependencies: {}
# [doc = " Asserts a [`String`] snapshot."] # [doc = ""] # [doc = " This is the simplest of all assertion methods."] # [doc = " It accepts any value that implements [`Display`](std::fmt::Display)."] # [doc = ""] # [doc = " ```no_run"] # [doc = " # use insta::*;"] # [doc = " // implicitly named"] # [doc = " assert_snapshot!(\"reference value to snapshot\");"] # [doc = " // named"] # [doc = " assert_snapshot!(\"snapshot_name\", \"reference value to snapshot\");"] # [doc = " // inline"] # [doc = " assert_snapshot!(\"reference value\", @\"reference value\");"] # [doc = " ```"] # [doc = ""] # [doc = " Optionally a third argument can be given as an expression to be stringified"] # [doc = " as the debug expression.  For more information on this, check out"] # [doc = " <https://insta.rs/docs/snapshot-types/>."] # [macro_export] macro_rules ! assert_snapshot { ($ ($ arg : tt) *) => { $ crate :: _assert_snapshot_base ! (transform =| v | $ crate :: _macro_support :: format ! ("{}" , v) , $ ($ arg) *) } ; }
};
}
