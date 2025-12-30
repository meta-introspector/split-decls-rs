// Generated macro for invalid_window_bits (function)
macro_rules! Depcrate_lib_oxideinvalid_window_bits {
() => {
// Module: crate::lib_oxide
// Provides: {"invalid_window_bits"}
// Dependencies: {}
# [doc = " Returns true if the window_bits parameter is valid."] fn invalid_window_bits (window_bits : i32) -> bool { (window_bits != MZ_DEFAULT_WINDOW_BITS) && (- window_bits != MZ_DEFAULT_WINDOW_BITS) }
};
}
