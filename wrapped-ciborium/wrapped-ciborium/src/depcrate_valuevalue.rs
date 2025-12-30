// Generated macro for Value (enum)
macro_rules! Depcrate_valueValue {
() => {
// Module: crate::value
// Provides: {"Value"}
// Dependencies: {}
# [doc = " A representation of a dynamic CBOR value that can be handled dynamically"] # [non_exhaustive] # [derive (Clone , Debug , PartialEq , PartialOrd)] pub enum Value { # [doc = " An integer"] Integer (Integer) , # [doc = " Bytes"] Bytes (Vec < u8 >) , # [doc = " A float"] Float (f64) , # [doc = " A string"] Text (String) , # [doc = " A boolean"] Bool (bool) , # [doc = " Null"] Null , # [doc = " Tag"] Tag (u64 , Box < Value >) , # [doc = " An array"] Array (Vec < Value >) , # [doc = " A map"] Map (Vec < (Value , Value) >) , }
};
}
