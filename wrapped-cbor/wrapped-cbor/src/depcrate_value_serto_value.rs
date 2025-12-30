// Generated macro for to_value (function)
macro_rules! Depcrate_value_serto_value {
() => {
// Module: crate::value::ser
// Provides: {"to_value"}
// Dependencies: {}
# [doc = " Convert a `T` into `serde_cbor::Value` which is an enum that can represent"] # [doc = " any valid CBOR data."] # [doc = ""] # [doc = " ```rust"] # [doc = " extern crate serde;"] # [doc = ""] # [doc = " #[macro_use]"] # [doc = " extern crate serde_derive;"] # [doc = " extern crate serde_cbor;"] # [doc = ""] # [doc = " use std::error::Error;"] # [doc = ""] # [doc = " #[derive(Serialize)]"] # [doc = " struct User {"] # [doc = "     fingerprint: String,"] # [doc = "     location: String,"] # [doc = " }"] # [doc = ""] # [doc = " fn main() {"] # [doc = "     let u = User {"] # [doc = "         fingerprint: \"0xF9BA143B95FF6D82\".to_owned(),"] # [doc = "         location: \"Menlo Park, CA\".to_owned(),"] # [doc = "     };"] # [doc = ""] # [doc = "     let v = serde_cbor::value::to_value(u).unwrap();"] # [doc = " }"] # [doc = " ```"] # [allow (clippy :: needless_pass_by_value)] pub fn to_value < T > (value : T) -> Result < Value , Error > where T : Serialize , { value . serialize (Serializer) }
};
}
