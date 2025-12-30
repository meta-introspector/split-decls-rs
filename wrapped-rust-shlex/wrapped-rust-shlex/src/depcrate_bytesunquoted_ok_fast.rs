// Generated macro for unquoted_ok_fast (function)
macro_rules! Depcrate_bytesunquoted_ok_fast {
() => {
// Module: crate::bytes
// Provides: {"unquoted_ok_fast"}
// Dependencies: {}
# [doc = " Optimized version of `unquoted_ok`."] fn unquoted_ok_fast (c : u8) -> bool { const UNQUOTED_OK_MASK : u128 = { let mut c = 0u8 ; let mut mask = 0u128 ; while c < 0x80 { if unquoted_ok (c) { mask |= 1u128 << c ; } c += 1 ; } mask } ; ((UNQUOTED_OK_MASK >> c) & 1) != 0 }
};
}
