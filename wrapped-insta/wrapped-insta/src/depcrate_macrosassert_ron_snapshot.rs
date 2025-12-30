// Generated macro for assert_ron_snapshot (macro)
macro_rules! Depcrate_macrosassert_ron_snapshot {
() => {
// Module: crate::macros
// Provides: {"assert_ron_snapshot"}
// Dependencies: {}
# [doc = " Asserts a [`serde::Serialize`] snapshot in RON format."] # [doc = ""] # [doc = " **Feature:** `ron` (disabled by default)"] # [doc = ""] # [doc = " This works exactly like [`assert_yaml_snapshot!`](crate::assert_yaml_snapshot!)"] # [doc = " but serializes in [RON](https://github.com/ron-rs/ron/) format instead of"] # [doc = " YAML which retains some type information for more accurate comparisons."] # [doc = ""] # [doc = " Example:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # use insta::*;"] # [doc = " assert_ron_snapshot!(vec![1, 2, 3]);"] # [doc = " ```"] # [doc = ""] # [doc = " The third argument to the macro can be an object expression for redaction."] # [doc = " It's in the form `{ selector => replacement }` or `match .. { selector => replacement }`."] # [doc = " For more information about redactions refer to the [redactions feature in"] # [doc = " the guide](https://insta.rs/docs/redactions/)."] # [doc = ""] # [doc = " The snapshot name is optional but can be provided as first argument."] # [cfg (feature = "ron")] # [cfg_attr (docsrs , doc (cfg (feature = "ron")))] # [macro_export] macro_rules ! assert_ron_snapshot { ($ ($ arg : tt) *) => { $ crate :: _assert_serialized_snapshot ! (format = Ron , $ ($ arg) *) ; } ; }
};
}
