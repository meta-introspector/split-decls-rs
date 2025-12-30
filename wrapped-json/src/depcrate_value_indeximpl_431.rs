// Generated macro for impl_431 (impl)
macro_rules! Depcrate_value_indeximpl_431 {
() => {
// Module: crate::value::index
// Provides: {"impl_431"}
// Dependencies: {}
impl < I > ops :: IndexMut < I > for Value where I : Index , { # [doc = " Write into a `serde_json::Value` using the syntax `value[0] = ...` or"] # [doc = " `value[\"k\"] = ...`."] # [doc = ""] # [doc = " If the index is a number, the value must be an array of length bigger"] # [doc = " than the index. Indexing into a value that is not an array or an array"] # [doc = " that is too small will panic."] # [doc = ""] # [doc = " If the index is a string, the value must be an object or null which is"] # [doc = " treated like an empty object. If the key is not already present in the"] # [doc = " object, it will be inserted with a value of null. Indexing into a value"] # [doc = " that is neither an object nor null will panic."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # use serde_json::json;"] # [doc = " #"] # [doc = " let mut data = json!({ \"x\": 0 });"] # [doc = ""] # [doc = " // replace an existing key"] # [doc = " data[\"x\"] = json!(1);"] # [doc = ""] # [doc = " // insert a new key"] # [doc = " data[\"y\"] = json!([false, false, false]);"] # [doc = ""] # [doc = " // replace an array value"] # [doc = " data[\"y\"][0] = json!(true);"] # [doc = ""] # [doc = " // inserted a deeply nested key"] # [doc = " data[\"a\"][\"b\"][\"c\"][\"d\"] = json!(true);"] # [doc = ""] # [doc = " println!(\"{}\", data);"] # [doc = " ```"] fn index_mut (& mut self , index : I) -> & mut Value { index . index_or_insert (self) } }
};
}
