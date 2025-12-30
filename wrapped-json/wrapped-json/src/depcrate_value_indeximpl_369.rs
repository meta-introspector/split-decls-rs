// Generated macro for impl_369 (impl)
macro_rules! Depcrate_value_indeximpl_369 {
() => {
// Module: crate::value::index
// Provides: {"impl_369"}
// Dependencies: {}
impl < I > ops :: Index < I > for Value where I : Index , { type Output = Value ; # [doc = " Index into a `serde_json::Value` using the syntax `value[0]` or"] # [doc = " `value[\"k\"]`."] # [doc = ""] # [doc = " Returns `Value::Null` if the type of `self` does not match the type of"] # [doc = " the index, for example if the index is a string and `self` is an array"] # [doc = " or a number. Also returns `Value::Null` if the given key does not exist"] # [doc = " in the map or the given index is not within the bounds of the array."] # [doc = ""] # [doc = " For retrieving deeply nested values, you should have a look at the"] # [doc = " `Value::pointer` method."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use serde_json::json;"] # [doc = " #"] # [doc = " let data = json!({"] # [doc = "     \"x\": {"] # [doc = "         \"y\": [\"z\", \"zz\"]"] # [doc = "     }"] # [doc = " });"] # [doc = ""] # [doc = " assert_eq!(data[\"x\"][\"y\"], json!([\"z\", \"zz\"]));"] # [doc = " assert_eq!(data[\"x\"][\"y\"][0], json!(\"z\"));"] # [doc = ""] # [doc = " assert_eq!(data[\"a\"], json!(null)); // returns null for undefined values"] # [doc = " assert_eq!(data[\"a\"][\"b\"], json!(null)); // does not panic"] # [doc = " ```"] fn index (& self , index : I) -> & Value { static NULL : Value = Value :: Null ; index . index_into (self) . unwrap_or (& NULL) } }
};
}
