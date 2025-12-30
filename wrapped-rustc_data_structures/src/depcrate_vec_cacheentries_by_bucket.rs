// Generated macro for ENTRIES_BY_BUCKET (const)
macro_rules! Depcrate_vec_cacheENTRIES_BY_BUCKET {
() => {
// Module: crate::vec_cache
// Provides: {"ENTRIES_BY_BUCKET"}
// Dependencies: {}
const ENTRIES_BY_BUCKET : [usize ; 21] = { let mut entries = [0 ; 21] ; let mut key = 0 ; loop { let si = SlotIndex :: from_index (key) ; entries [si . bucket_idx] = si . entries ; if key == 0 { key = 1 ; } else if key == (1 << 31) { break ; } else { key <<= 1 ; } } entries } ;
};
}
