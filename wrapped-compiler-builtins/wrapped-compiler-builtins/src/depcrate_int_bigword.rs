// Generated macro for word (macro)
macro_rules! Depcrate_int_bigword {
() => {
// Module: crate::int::big
// Provides: {"word"}
// Dependencies: {}
macro_rules ! word { (1 , $ val : expr) => { (($ val >> (32 * 3)) & Self :: from (WORD_LO_MASK)) as u64 } ; (2 , $ val : expr) => { (($ val >> (32 * 2)) & Self :: from (WORD_LO_MASK)) as u64 } ; (3 , $ val : expr) => { (($ val >> (32 * 1)) & Self :: from (WORD_LO_MASK)) as u64 } ; (4 , $ val : expr) => { (($ val >> (32 * 0)) & Self :: from (WORD_LO_MASK)) as u64 } ; }
};
}
