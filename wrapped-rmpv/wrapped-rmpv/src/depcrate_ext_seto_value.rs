// Generated macro for to_value (function)
macro_rules! Depcrate_ext_seto_value {
() => {
// Module: crate::ext::se
// Provides: {"to_value"}
// Dependencies: {}
# [doc = " Convert a `T` into `rmpv::Value` which is an enum that can represent any valid MessagePack data."] # [doc = ""] # [doc = " This conversion can fail if `T`'s implementation of `Serialize` decides to fail."] # [doc = ""] # [doc = " ```rust"] # [doc = " # use rmpv::Value;"] # [doc = ""] # [doc = " let val = rmpv::ext::to_value(\"John Smith\").unwrap();"] # [doc = ""] # [doc = " assert_eq!(Value::String(\"John Smith\".into()), val);"] # [doc = " ```"] # [inline] pub fn to_value < T : Serialize > (value : T) -> Result < Value , Error > { value . serialize (Serializer) }
};
}
