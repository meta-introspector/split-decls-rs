// Generated macro for remove_null_values (function)
macro_rules! Depcrate_server_utilsremove_null_values {
() => {
// Module: crate::server::utils
// Provides: {"remove_null_values"}
// Dependencies: {}
# [doc = " Remove null values from JSON (TOML doesn't support null)"] pub fn remove_null_values (value : & mut serde_json :: Value) { match value { serde_json :: Value :: Object (map) => { map . retain (| _ , v | ! v . is_null ()) ; for (_ , v) in map . iter_mut () { remove_null_values (v) ; } } serde_json :: Value :: Array (arr) => { for item in arr . iter_mut () { remove_null_values (item) ; } } _ => { } } }
};
}
