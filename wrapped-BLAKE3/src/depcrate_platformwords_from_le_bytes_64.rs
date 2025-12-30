// Generated macro for words_from_le_bytes_64 (function)
macro_rules! Depcrate_platformwords_from_le_bytes_64 {
() => {
// Module: crate::platform
// Provides: {"words_from_le_bytes_64"}
// Dependencies: {}
# [inline (always)] pub fn words_from_le_bytes_64 (bytes : & [u8 ; 64]) -> [u32 ; 16] { let mut out = [0 ; 16] ; out [0] = u32 :: from_le_bytes (* array_ref ! (bytes , 0 * 4 , 4)) ; out [1] = u32 :: from_le_bytes (* array_ref ! (bytes , 1 * 4 , 4)) ; out [2] = u32 :: from_le_bytes (* array_ref ! (bytes , 2 * 4 , 4)) ; out [3] = u32 :: from_le_bytes (* array_ref ! (bytes , 3 * 4 , 4)) ; out [4] = u32 :: from_le_bytes (* array_ref ! (bytes , 4 * 4 , 4)) ; out [5] = u32 :: from_le_bytes (* array_ref ! (bytes , 5 * 4 , 4)) ; out [6] = u32 :: from_le_bytes (* array_ref ! (bytes , 6 * 4 , 4)) ; out [7] = u32 :: from_le_bytes (* array_ref ! (bytes , 7 * 4 , 4)) ; out [8] = u32 :: from_le_bytes (* array_ref ! (bytes , 8 * 4 , 4)) ; out [9] = u32 :: from_le_bytes (* array_ref ! (bytes , 9 * 4 , 4)) ; out [10] = u32 :: from_le_bytes (* array_ref ! (bytes , 10 * 4 , 4)) ; out [11] = u32 :: from_le_bytes (* array_ref ! (bytes , 11 * 4 , 4)) ; out [12] = u32 :: from_le_bytes (* array_ref ! (bytes , 12 * 4 , 4)) ; out [13] = u32 :: from_le_bytes (* array_ref ! (bytes , 13 * 4 , 4)) ; out [14] = u32 :: from_le_bytes (* array_ref ! (bytes , 14 * 4 , 4)) ; out [15] = u32 :: from_le_bytes (* array_ref ! (bytes , 15 * 4 , 4)) ; out }
};
}
