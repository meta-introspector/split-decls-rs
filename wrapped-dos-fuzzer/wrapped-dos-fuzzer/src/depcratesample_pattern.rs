// Generated macro for sample_pattern (function)
macro_rules! Depcratesample_pattern {
() => {
// Module: crate
// Provides: {"sample_pattern"}
// Dependencies: {}
# [doc = " Returns the number of repeated patterns."] fn sample_pattern (pattern : & Pattern , buf : & mut String , sample_size : usize , sample_count : usize ,) -> usize { let target_byte_count = sample_size * NUM_BYTES / sample_count ; let target_repeat_bytes = target_byte_count - buf . len () - pattern . suffix . len () ; let num_repeats = target_repeat_bytes / pattern . repeating_pattern . len () ; buf . extend (std :: iter :: repeat (& pattern . repeating_pattern [..]) . take (num_repeats)) ; buf . push_str (& pattern . suffix) ; num_repeats }
};
}
