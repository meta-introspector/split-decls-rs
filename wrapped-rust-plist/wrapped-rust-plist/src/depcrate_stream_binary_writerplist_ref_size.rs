// Generated macro for plist_ref_size (function)
macro_rules! Depcrate_stream_binary_writerplist_ref_size {
() => {
// Module: crate::stream::binary_writer
// Provides: {"plist_ref_size"}
// Dependencies: {}
fn plist_ref_size (max_value : usize) -> u8 { let significant_bits = 64 - (max_value as u64) . leading_zeros () as u8 ; let significant_bytes = (significant_bits + 7) / 8 ; significant_bytes . next_power_of_two () }
};
}
