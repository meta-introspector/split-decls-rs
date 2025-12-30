// Generated macro for VersionedG1Multiplication (enum)
macro_rules! Depcrate_multiplicationVersionedG1Multiplication {
() => {
// Module: crate::multiplication
// Provides: {"VersionedG1Multiplication"}
// Dependencies: {}
# [doc = " The version enum used to version changes to the `alt_bn128_g1_multiplication` syscall."] # [cfg (not (target_os = "solana"))] pub enum VersionedG1Multiplication { V0 , # [doc = " SIMD-0222 - Fix alt-bn128-multiplication Syscall Length Check"] V1 , }
};
}
