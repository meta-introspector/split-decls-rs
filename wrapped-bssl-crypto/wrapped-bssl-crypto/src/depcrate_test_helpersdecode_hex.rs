// Generated macro for decode_hex (function)
macro_rules! Depcrate_test_helpersdecode_hex {
() => {
// Module: crate::test_helpers
// Provides: {"decode_hex"}
// Dependencies: {}
# [allow (clippy :: expect_used , clippy :: unwrap_used , clippy :: indexing_slicing)] pub (crate) fn decode_hex < const N : usize > (s : & str) -> [u8 ; N] { (0 .. s . len ()) . step_by (2) . map (| i | u8 :: from_str_radix (& s [i .. i + 2] , 16) . expect ("Invalid hex string")) . collect :: < Vec < u8 > > () . as_slice () . try_into () . unwrap () }
};
}
