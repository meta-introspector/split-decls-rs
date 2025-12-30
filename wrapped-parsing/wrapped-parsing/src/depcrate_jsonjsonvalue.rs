// Generated macro for JsonValue (enum)
macro_rules! Depcrate_jsonJsonValue {
() => {
// Module: crate::json
// Provides: {"JsonValue"}
// Dependencies: {}
# [derive (Debug , PartialEq , Clone)] pub enum JsonValue { Null , Bool (bool) , Str (String) , Num (f64) , Array (Vec < JsonValue >) , Object (HashMap < String , JsonValue >) , }
};
}
