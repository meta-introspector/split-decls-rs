// Generated macro for bytes_are_equal (function)
macro_rules! Depcrate_bb_bytesbytes_are_equal {
() => {
// Module: crate::bb::bytes
// Provides: {"bytes_are_equal"}
// Dependencies: {}
# [must_use] pub fn bytes_are_equal (a : & [u8] , b : & [u8]) -> BoolMask { let len = a . len () ; if b . len () != len { return BoolMask :: FALSE ; } let (a , a_rem) = a . as_chunks () ; let (b , b_rem) = b . as_chunks () ; let mut acc = a . iter () . copied () . map (Word :: from_le_bytes) . zip (b . iter () . copied () . map (Word :: from_le_bytes)) . fold (0 , | acc , (a , b) | acc | (a ^ b)) ; if ! a_rem . is_empty () { # [allow (clippy :: into_iter_on_ref)] let rem = a_rem . into_iter () . copied () . map (Word :: from) . zip (b_rem . into_iter () . copied () . map (Word :: from)) . fold (0 , | acc , (a , b) | acc | (a ^ b)) ; acc |= rem ; } WordOps :: is_zero (acc) }
};
}
