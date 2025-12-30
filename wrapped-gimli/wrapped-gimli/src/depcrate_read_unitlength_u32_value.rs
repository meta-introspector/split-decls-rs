// Generated macro for length_u32_value (function)
macro_rules! Depcrate_read_unitlength_u32_value {
() => {
// Module: crate::read::unit
// Provides: {"length_u32_value"}
// Dependencies: {}
fn length_u32_value < R : Reader > (input : & mut R) -> Result < R > { let len = input . read_u32 () . map (R :: Offset :: from_u32) ? ; input . split (len) }
};
}
