// Generated macro for h1 (function)
macro_rules! Depcrate_rawh1 {
() => {
// Module: crate::raw
// Provides: {"h1"}
// Dependencies: {}
# [doc = " Primary hash function, used to select the initial bucket to probe from."] # [inline] # [allow (clippy :: cast_possible_truncation)] fn h1 (hash : u64) -> usize { hash as usize }
};
}
