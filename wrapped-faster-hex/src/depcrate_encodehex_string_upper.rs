// Generated macro for hex_string_upper (function)
macro_rules! Depcrate_encodehex_string_upper {
() => {
// Module: crate::encode
// Provides: {"hex_string_upper"}
// Dependencies: {}
# [cfg (not (feature = "alloc"))] pub fn hex_string_upper < const N : usize > (src : & [u8]) -> String < N > { hex_string_custom_case (src , true) }
};
}
