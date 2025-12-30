// Generated macro for validated_attr_value_byte (function)
macro_rules! Depcrate_parsevalidated_attr_value_byte {
() => {
// Module: crate::parse
// Provides: {"validated_attr_value_byte"}
// Dependencies: {}
fn validated_attr_value_byte (byte : u8) -> Result < u8 , Error > { if is_valid_attr_value (byte) { Ok (byte) } else { Err (Error :: InvalidAttributeValue { character : byte as char , }) } }
};
}
