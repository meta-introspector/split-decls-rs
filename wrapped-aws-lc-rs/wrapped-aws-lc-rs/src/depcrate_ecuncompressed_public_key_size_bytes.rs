// Generated macro for uncompressed_public_key_size_bytes (function)
macro_rules! Depcrate_ecuncompressed_public_key_size_bytes {
() => {
// Module: crate::ec
// Provides: {"uncompressed_public_key_size_bytes"}
// Dependencies: {}
# [inline] pub (crate) const fn uncompressed_public_key_size_bytes (curve_field_bits : usize) -> usize { 1 + 2 * ((curve_field_bits + 7) / 8) }
};
}
