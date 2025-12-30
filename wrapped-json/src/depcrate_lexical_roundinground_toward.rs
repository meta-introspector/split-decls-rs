// Generated macro for round_toward (function)
macro_rules! Depcrate_lexical_roundinground_toward {
() => {
// Module: crate::lexical::rounding
// Provides: {"round_toward"}
// Dependencies: {}
# [inline] fn round_toward (fp : & mut ExtendedFloat , shift : i32) -> bool { let mask : u64 = lower_n_mask (shift as u64) ; let truncated_bits = fp . mant & mask ; overflowing_shr (fp , shift) ; truncated_bits != 0 }
};
}
