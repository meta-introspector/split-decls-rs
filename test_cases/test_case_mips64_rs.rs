// MINIMAL TEST CASE for parsing failure in: ../rust/library/std_detect/src/detect/arch/mips64.rs
// Error: expected square brackets
// Problematic line: line 3

//! Run-time feature detection on MIPS64.

features! {
    @TARGET: mips64;
    @CFG: target_arch = "mips64";
    @MACRO_NAME: is_mips64_feature_detected;
