// Generated macro for assert_csv_snapshot (macro)
macro_rules! Depcrate_macrosassert_csv_snapshot {
() => {
// Module: crate::macros
// Provides: {"assert_csv_snapshot"}
// Dependencies: {}
# [doc = " Asserts a [`serde::Serialize`] snapshot in CSV format."] # [doc = ""] # [doc = " **Feature:** `csv` (disabled by default)"] # [doc = ""] # [doc = " This works exactly like [`assert_yaml_snapshot!`](crate::assert_yaml_snapshot!)"] # [doc = " but serializes in [CSV](https://github.com/burntsushi/rust-csv) format instead of"] # [doc = " YAML."] # [doc = ""] # [doc = " Example:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " insta::assert_csv_snapshot!(vec![1, 2, 3]);"] # [doc = " ```"] # [doc = ""] # [doc = " The third argument to the macro can be an object expression for redaction."] # [doc = " It's in the form `{ selector => replacement }` or `match .. { selector => replacement }`."] # [doc = " For more information about redactions refer to the [redactions feature in"] # [doc = " the guide](https://insta.rs/docs/redactions/)."] # [doc = ""] # [doc = " The snapshot name is optional but can be provided as first argument."] # [cfg (feature = "csv")] # [cfg_attr (docsrs , doc (cfg (feature = "csv")))] # [macro_export] macro_rules ! assert_csv_snapshot { ($ ($ arg : tt) *) => { $ crate :: _assert_serialized_snapshot ! (format = Csv , $ ($ arg) *) ; } ; }
};
}
