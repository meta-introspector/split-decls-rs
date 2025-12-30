// Generated macro for impl_7 (impl)
macro_rules! Depcrate_dictionaryimpl_7 {
() => {
// Module: crate::dictionary
// Provides: {"impl_7"}
// Dependencies: {}
# [doc = " Access an element of this dictionary. Panics if the given key is not present in the dictionary."] # [doc = ""] # [doc = " ```"] # [doc = " # use plist::Value;"] # [doc = " #"] # [doc = " # let val = &Value::String(\"\".to_owned());"] # [doc = " # let _ ="] # [doc = " match *val {"] # [doc = "     Value::Array(ref arr) => arr[0].as_string(),"] # [doc = "     Value::Dictionary(ref dict) => dict[\"type\"].as_string(),"] # [doc = "     Value::String(ref s) => Some(s.as_str()),"] # [doc = "     _ => None,"] # [doc = " }"] # [doc = " # ;"] # [doc = " ```"] impl ops :: Index < & str > for Dictionary { type Output = Value ; fn index (& self , index : & str) -> & Value { self . map . index (index) } }
};
}
