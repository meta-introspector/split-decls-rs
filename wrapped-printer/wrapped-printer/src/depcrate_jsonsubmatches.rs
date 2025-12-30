// Generated macro for SubMatches (enum)
macro_rules! Depcrate_jsonSubMatches {
() => {
// Module: crate::json
// Provides: {"SubMatches"}
// Dependencies: {}
# [doc = " SubMatches represents a set of matches in a contiguous range of bytes."] # [doc = ""] # [doc = " A simpler representation for this would just simply be `Vec<SubMatch>`,"] # [doc = " but the common case is exactly one match per range of bytes, which we"] # [doc = " specialize here using a fixed size array without any allocation."] enum SubMatches < 'a > { Empty , Small ([jsont :: SubMatch < 'a > ; 1]) , Big (Vec < jsont :: SubMatch < 'a > >) , }
};
}
