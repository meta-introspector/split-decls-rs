// Generated macro for decode_hex_into_vec (function)
macro_rules! Depcrate_test_helpersdecode_hex_into_vec {
() => {
// Module: crate::test_helpers
// Provides: {"decode_hex_into_vec"}
// Dependencies: {}
# [allow (clippy :: expect_used , clippy :: unwrap_used , clippy :: indexing_slicing)] pub (crate) fn decode_hex_into_vec (s : & str) -> Vec < u8 > { (0 .. s . len ()) . step_by (2) . map (| i | u8 :: from_str_radix (& s [i .. i + 2] , 16) . expect ("Invalid hex string")) . collect :: < Vec < u8 > > () }
};
}
