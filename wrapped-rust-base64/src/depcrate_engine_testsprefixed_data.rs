// Generated macro for prefixed_data (function)
macro_rules! Depcrate_engine_testsprefixed_data {
() => {
// Module: crate::engine::tests
// Provides: {"prefixed_data"}
// Dependencies: {}
fn prefixed_data < 'i > (input_with_prefix : & 'i mut String , prefix_len : usize , data : & str) -> & 'i str { input_with_prefix . truncate (prefix_len) ; input_with_prefix . push_str (data) ; input_with_prefix . as_str () }
};
}
