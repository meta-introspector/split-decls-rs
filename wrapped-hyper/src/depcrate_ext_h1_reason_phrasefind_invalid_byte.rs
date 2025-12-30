// Generated macro for find_invalid_byte (function)
macro_rules! Depcrate_ext_h1_reason_phrasefind_invalid_byte {
() => {
// Module: crate::ext::h1_reason_phrase
// Provides: {"find_invalid_byte"}
// Dependencies: {}
const fn find_invalid_byte (bytes : & [u8]) -> Option < u8 > { let mut i = 0 ; while i < bytes . len () { let b = bytes [i] ; if ! is_valid_byte (b) { return Some (b) ; } i += 1 ; } None }
};
}
