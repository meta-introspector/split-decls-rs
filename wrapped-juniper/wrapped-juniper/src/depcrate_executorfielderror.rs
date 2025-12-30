// Generated macro for FieldError (struct)
macro_rules! Depcrate_executorFieldError {
() => {
// Module: crate::executor
// Provides: {"FieldError"}
// Dependencies: {}
# [doc = " Error type for errors that occur during field resolution"] # [doc = ""] # [doc = " Field errors are represented by a human-readable error message and an"] # [doc = " optional `Value` structure containing additional information."] # [doc = ""] # [doc = " They can be converted to from any type that implements `std::fmt::Display`,"] # [doc = " which makes error chaining with the `?` operator a breeze:"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use juniper::{FieldError, ScalarValue};"] # [doc = " fn get_string(data: Vec<u8>) -> Result<String, FieldError>"] # [doc = " {"] # [doc = "     let s = String::from_utf8(data)?;"] # [doc = "     Ok(s)"] # [doc = " }"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq)] pub struct FieldError < S = DefaultScalarValue > { message : String , extensions : Value < S > , }
};
}
