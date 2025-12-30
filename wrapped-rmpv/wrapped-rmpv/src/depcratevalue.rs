// Generated macro for Value (enum)
macro_rules! DepcrateValue {
() => {
// Module: crate
// Provides: {"Value"}
// Dependencies: {}
# [doc = " Represents any valid MessagePack value."] # [derive (Clone , Debug , PartialEq)] pub enum Value { # [doc = " Nil represents nil."] Nil , # [doc = " Boolean represents true or false."] Boolean (bool) , # [doc = " Integer represents an integer."] # [doc = ""] # [doc = " A value of an `Integer` object is limited from `-(2^63)` upto `(2^64)-1`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use rmpv::Value;"] # [doc = ""] # [doc = " assert_eq!(42, Value::from(42).as_i64().unwrap());"] # [doc = " ```"] Integer (Integer) , # [doc = " A 32-bit floating point number."] F32 (f32) , # [doc = " A 64-bit floating point number."] F64 (f64) , # [doc = " String extending Raw type represents a UTF-8 string."] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " String objects may contain invalid byte sequence and the behavior of a deserializer depends"] # [doc = " on the actual implementation when it received invalid byte sequence. Deserializers should"] # [doc = " provide functionality to get the original byte array so that applications can decide how to"] # [doc = " handle the object"] String (Utf8String) , # [doc = " Binary extending Raw type represents a byte array."] Binary (Vec < u8 >) , # [doc = " Array represents a sequence of objects."] Array (Vec < Value >) , # [doc = " Map represents key-value pairs of objects."] Map (Vec < (Value , Value) >) , # [doc = " Extended implements Extension interface: represents a tuple of type information and a byte"] # [doc = " array where type information is an integer whose meaning is defined by applications."] Ext (i8 , Vec < u8 >) , }
};
}
