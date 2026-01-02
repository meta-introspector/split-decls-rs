// MINIMAL TEST CASE for parsing failure in: ../rust/library/std_detect/src/detect/arch/mips.rs
// Error: expected square brackets
// Problematic line: line 3

//! Run-time feature detection on MIPS.

features! {
    @TARGET: mips;
    @CFG: target_arch = "mips";
    @MACRO_NAME: is_mips_feature_detected;
