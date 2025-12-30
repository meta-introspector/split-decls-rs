// Generated macro for BIGINT_BITS (const)
macro_rules! Depcrate_bigintBIGINT_BITS {
() => {
// Module: crate::bigint
// Provides: {"BIGINT_BITS"}
// Dependencies: {}
# [doc = " Number of bits in a Bigint."] # [doc = ""] # [doc = " This needs to be at least the number of bits required to store"] # [doc = " a Bigint, which is `log2(radix**digits)`."] # [doc = " ≅ 3600 for base-10, rounded-up."] pub const BIGINT_BITS : usize = 4000 ;
};
}
