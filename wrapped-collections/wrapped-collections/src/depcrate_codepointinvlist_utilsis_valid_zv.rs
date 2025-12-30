// Generated macro for is_valid_zv (function)
macro_rules! Depcrate_codepointinvlist_utilsis_valid_zv {
() => {
// Module: crate::codepointinvlist::utils
// Provides: {"is_valid_zv"}
// Dependencies: {}
# [doc = " Returns whether the vector is sorted ascending non inclusive, of even length,"] # [doc = " and within the bounds of `0x0 -> 0x10FFFF + 1` inclusive."] # [expect (clippy :: indexing_slicing)] # [expect (clippy :: unwrap_used)] pub fn is_valid_zv (inv_list_zv : & ZeroVec < '_ , PotentialCodePoint >) -> bool { inv_list_zv . is_empty () || (inv_list_zv . len () % 2 == 0 && inv_list_zv . as_ule_slice () . windows (2) . all (| chunk | { < PotentialCodePoint as AsULE > :: from_unaligned (chunk [0]) < < PotentialCodePoint as AsULE > :: from_unaligned (chunk [1]) }) && u32 :: from (inv_list_zv . last () . unwrap ()) <= char :: MAX as u32 + 1) }
};
}
