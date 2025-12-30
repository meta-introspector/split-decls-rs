// Generated macro for Deserializer (struct)
macro_rules! Depcrate_deDeserializer {
() => {
// Module: crate::de
// Provides: {"Deserializer"}
// Dependencies: {}
# [doc = " A structure that deserializes JSON into Rust values."] pub struct Deserializer < R > { read : R , scratch : Vec < u8 > , remaining_depth : u8 , # [cfg (feature = "float_roundtrip")] single_precision : bool , # [cfg (feature = "unbounded_depth")] disable_recursion_limit : bool , }
};
}
