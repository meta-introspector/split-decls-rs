// Generated macro for from_value (function)
macro_rules! Depcrate_value_defrom_value {
() => {
// Module: crate::value::de
// Provides: {"from_value"}
// Dependencies: {}
# [doc = " Convert a `serde_cbor::Value` into a type `T`"] # [allow (clippy :: needless_pass_by_value)] pub fn from_value < T > (value : Value) -> Result < T , crate :: error :: Error > where T : de :: DeserializeOwned , { let buf = crate :: to_vec (& value) ? ; crate :: from_slice (buf . as_slice ()) }
};
}
