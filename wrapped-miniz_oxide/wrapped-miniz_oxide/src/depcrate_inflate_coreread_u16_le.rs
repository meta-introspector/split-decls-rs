// Generated macro for read_u16_le (function)
macro_rules! Depcrate_inflate_coreread_u16_le {
() => {
// Module: crate::inflate::core
// Provides: {"read_u16_le"}
// Dependencies: {}
# [doc = " Read an le u16 value from the slice iterator."] # [doc = ""] # [doc = " # Panics"] # [doc = " Panics if there are less than two bytes left."] # [inline] fn read_u16_le (iter : & mut InputWrapper) -> u16 { let ret = { let two_bytes = iter . as_slice () [.. 2] . try_into () . unwrap_or_default () ; u16 :: from_le_bytes (two_bytes) } ; iter . advance (2) ; ret }
};
}
