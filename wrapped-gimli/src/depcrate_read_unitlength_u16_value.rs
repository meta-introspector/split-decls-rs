// Generated macro for length_u16_value (function)
macro_rules! Depcrate_read_unitlength_u16_value {
() => {
// Module: crate::read::unit
// Provides: {"length_u16_value"}
// Dependencies: {}
fn length_u16_value < R : Reader > (input : & mut R) -> Result < R > { let len = input . read_u16 () . map (R :: Offset :: from_u16) ? ; input . split (len) }
};
}
