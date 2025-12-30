// Generated macro for swap_if_opposite_endian (function)
macro_rules! Depcrate_handlesswap_if_opposite_endian {
() => {
// Module: crate::handles
// Provides: {"swap_if_opposite_endian"}
// Dependencies: {}
# [inline (always)] fn swap_if_opposite_endian < E : Endian > (unit : u16) -> u16 { if E :: OPPOSITE_ENDIAN { unit . swap_bytes () } else { unit } }
};
}
