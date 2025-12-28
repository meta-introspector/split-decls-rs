macro_rules! deps {
    () => {
        Value!();
        Error!();
    };
}

macro_rules! Serializer {
    () => {
        deps!();
        # [doc = " Serializer whose output is a `Value`."] # [doc = ""] # [doc = " This is the serializer that backs [`serde_json::to_value`][crate::to_value]."] # [doc = " Unlike the main serde_json serializer which goes from some serializable"] # [doc = " value of type `T` to JSON text, this one goes from `T` to"] # [doc = " `serde_json::Value`."] # [doc = ""] # [doc = " The `to_value` function is implementable as:"] # [doc = ""] # [doc = " ```"] # [doc = " use serde::Serialize;"] # [doc = " use serde_json::{Error, Value};"] # [doc = ""] # [doc = " pub fn to_value<T>(input: T) -> Result<Value, Error>"] # [doc = " where"] # [doc = "     T: Serialize,"] # [doc = " {"] # [doc = "     input.serialize(serde_json::value::Serializer)"] # [doc = " }"] # [doc = " ```"] pub struct Serializer ;
    };
}

Serializer!();