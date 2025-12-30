// Generated macro for assert_toml_snapshot (macro)
macro_rules! Depcrate_macrosassert_toml_snapshot {
() => {
// Module: crate::macros
// Provides: {"assert_toml_snapshot"}
// Dependencies: {}
# [doc = " Asserts a [`serde::Serialize`] snapshot in TOML format."] # [doc = ""] # [doc = " **Feature:** `toml` (disabled by default)"] # [doc = ""] # [doc = " This works exactly like [`assert_yaml_snapshot!`](crate::assert_yaml_snapshot!)"] # [doc = " but serializes in [TOML](https://github.com/alexcrichton/toml-rs) format instead of"] # [doc = " YAML.  Note that TOML cannot represent all values due to limitations in the"] # [doc = " format."] # [doc = ""] # [doc = " Example:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " insta::assert_toml_snapshot!(vec![1, 2, 3]);"] # [doc = " ```"] # [doc = ""] # [doc = " The third argument to the macro can be an object expression for redaction."] # [doc = " It's in the form `{ selector => replacement }` or `match .. { selector => replacement }`."] # [doc = " For more information about redactions refer to the [redactions feature in"] # [doc = " the guide](https://insta.rs/docs/redactions/)."] # [doc = ""] # [doc = " The snapshot name is optional but can be provided as first argument."] # [cfg (feature = "toml")] # [cfg_attr (docsrs , doc (cfg (feature = "toml")))] # [macro_export] macro_rules ! assert_toml_snapshot { ($ ($ arg : tt) *) => { $ crate :: _assert_serialized_snapshot ! (format = Toml , $ ($ arg) *) ; } ; }
};
}
