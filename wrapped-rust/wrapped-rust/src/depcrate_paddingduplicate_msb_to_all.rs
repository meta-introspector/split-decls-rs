// Generated macro for duplicate_msb_to_all (function)
macro_rules! Depcrate_paddingduplicate_msb_to_all {
() => {
// Module: crate::padding
// Provides: {"duplicate_msb_to_all"}
// Dependencies: {}
# [doc = " Returns the value of the input with the most-significant-bit copied to all"] # [doc = " of the bits."] fn duplicate_msb_to_all (a : u8) -> u8 { 0u8 . wrapping_sub (a >> 7) }
};
}
