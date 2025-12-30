// Generated macro for hex_string (function)
macro_rules! Depcrate_encodehex_string {
() => {
// Module: crate::encode
// Provides: {"hex_string"}
// Dependencies: {}
# [cfg (not (feature = "alloc"))] pub fn hex_string < const N : usize > (src : & [u8]) -> String < N > { hex_string_custom_case (src , false) }
};
}
