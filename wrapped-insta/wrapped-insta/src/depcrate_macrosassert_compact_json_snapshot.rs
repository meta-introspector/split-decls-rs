// Generated macro for assert_compact_json_snapshot (macro)
macro_rules! Depcrate_macrosassert_compact_json_snapshot {
() => {
// Module: crate::macros
// Provides: {"assert_compact_json_snapshot"}
// Dependencies: {}
# [doc = " Asserts a [`serde::Serialize`] snapshot in compact JSON format."] # [doc = ""] # [doc = " **Feature:** `json`"] # [doc = ""] # [doc = " This works exactly like [`assert_json_snapshot!`](crate::assert_json_snapshot!) but serializes into a single"] # [doc = " line for as long as the output is less than 120 characters.  This can be useful"] # [doc = " in cases where you are working with small result outputs but comes at the cost"] # [doc = " of slightly worse diffing behavior."] # [doc = ""] # [doc = " Example:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # use insta::*;"] # [doc = " assert_compact_json_snapshot!(vec![1, 2, 3]);"] # [doc = " ```"] # [doc = ""] # [doc = " The third argument to the macro can be an object expression for redaction."] # [doc = " It's in the form `{ selector => replacement }` or `match .. { selector => replacement }`."] # [doc = " For more information about redactions refer to the [redactions feature in"] # [doc = " the guide](https://insta.rs/docs/redactions/)."] # [doc = ""] # [doc = " The snapshot name is optional but can be provided as first argument."] # [cfg (feature = "json")] # [cfg_attr (docsrs , doc (cfg (feature = "json")))] # [macro_export] macro_rules ! assert_compact_json_snapshot { ($ ($ arg : tt) *) => { $ crate :: _assert_serialized_snapshot ! (format = JsonCompact , $ ($ arg) *) ; } ; }
};
}
