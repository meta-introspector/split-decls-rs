// Generated macro for is_valid_attr_value (function)
macro_rules! Depcrate_parseis_valid_attr_value {
() => {
// Module: crate::parse
// Provides: {"is_valid_attr_value"}
// Dependencies: {}
fn is_valid_attr_value (byte : u8) -> bool { byte . is_ascii_alphanumeric () || b",-_" . contains (& byte) }
};
}
