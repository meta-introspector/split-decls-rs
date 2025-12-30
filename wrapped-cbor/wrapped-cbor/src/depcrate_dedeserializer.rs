// Generated macro for Deserializer (struct)
macro_rules! Depcrate_deDeserializer {
() => {
// Module: crate::de
// Provides: {"Deserializer"}
// Dependencies: {}
# [doc = " A Serde `Deserialize`r of CBOR data."] # [derive (Debug)] pub struct Deserializer < R > { read : R , remaining_depth : u8 , accept_named : bool , accept_packed : bool , accept_standard_enums : bool , accept_legacy_enums : bool , }
};
}
