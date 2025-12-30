// Generated macro for length_uleb128_value (function)
macro_rules! Depcrate_read_unitlength_uleb128_value {
() => {
// Module: crate::read::unit
// Provides: {"length_uleb128_value"}
// Dependencies: {}
fn length_uleb128_value < R : Reader > (input : & mut R) -> Result < R > { let len = input . read_uleb128 () . and_then (R :: Offset :: from_u64) ? ; input . split (len) }
};
}
