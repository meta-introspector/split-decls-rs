// Generated macro for string_to_u32 (function)
macro_rules! Depcrate_x86_xml_parserstring_to_u32 {
() => {
// Module: crate::x86::xml_parser
// Provides: {"string_to_u32"}
// Dependencies: {}
fn string_to_u32 < 'de , D > (deserializer : D) -> Result < u32 , D :: Error > where D : Deserializer < 'de > , { let s = String :: deserialize (deserializer) ? ; return s . as_str () . parse :: < u32 > () . or (Ok (0u32)) ; }
};
}
