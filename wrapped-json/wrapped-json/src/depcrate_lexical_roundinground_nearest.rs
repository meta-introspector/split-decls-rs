// Generated macro for round_nearest (function)
macro_rules! Depcrate_lexical_roundinground_nearest {
() => {
// Module: crate::lexical::rounding
// Provides: {"round_nearest"}
// Dependencies: {}
# [inline] pub (crate) fn round_nearest (fp : & mut ExtendedFloat , shift : i32) -> (bool , bool) { let mask : u64 = lower_n_mask (shift as u64) ; let halfway : u64 = lower_n_halfway (shift as u64) ; let truncated_bits = fp . mant & mask ; let is_above = truncated_bits > halfway ; let is_halfway = truncated_bits == halfway ; overflowing_shr (fp , shift) ; (is_above , is_halfway) }
};
}
