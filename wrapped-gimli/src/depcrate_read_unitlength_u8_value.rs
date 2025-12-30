// Generated macro for length_u8_value (function)
macro_rules! Depcrate_read_unitlength_u8_value {
() => {
// Module: crate::read::unit
// Provides: {"length_u8_value"}
// Dependencies: {}
fn length_u8_value < R : Reader > (input : & mut R) -> Result < R > { let len = input . read_u8 () . map (R :: Offset :: from_u8) ? ; input . split (len) }
};
}
