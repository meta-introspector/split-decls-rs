// Generated macro for words_from_le_bytes_32 (function)
macro_rules! Depcrate_platformwords_from_le_bytes_32 {
() => {
// Module: crate::platform
// Provides: {"words_from_le_bytes_32"}
// Dependencies: {}
# [inline (always)] pub fn words_from_le_bytes_32 (bytes : & [u8 ; 32]) -> [u32 ; 8] { let mut out = [0 ; 8] ; out [0] = u32 :: from_le_bytes (* array_ref ! (bytes , 0 * 4 , 4)) ; out [1] = u32 :: from_le_bytes (* array_ref ! (bytes , 1 * 4 , 4)) ; out [2] = u32 :: from_le_bytes (* array_ref ! (bytes , 2 * 4 , 4)) ; out [3] = u32 :: from_le_bytes (* array_ref ! (bytes , 3 * 4 , 4)) ; out [4] = u32 :: from_le_bytes (* array_ref ! (bytes , 4 * 4 , 4)) ; out [5] = u32 :: from_le_bytes (* array_ref ! (bytes , 5 * 4 , 4)) ; out [6] = u32 :: from_le_bytes (* array_ref ! (bytes , 6 * 4 , 4)) ; out [7] = u32 :: from_le_bytes (* array_ref ! (bytes , 7 * 4 , 4)) ; out }
};
}
