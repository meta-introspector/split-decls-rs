// Generated macro for full_name (function)
macro_rules! Depcrate_genfull_name {
() => {
// Module: crate::gen
// Provides: {"full_name"}
// Dependencies: {}
# [doc = " Generate a full name."] pub fn full_name (rng : & mut SmallRng , len_lo : usize , len_hi : usize) -> String { format ! ("{} {}" , name (rng , len_lo , len_hi) , name (rng , len_lo , len_hi)) }
};
}
