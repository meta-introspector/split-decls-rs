// MINIMAL TEST CASE for parsing failure in: ../rust/library/std_detect/src/detect/arch/powerpc64.rs
// Error: expected square brackets
// Problematic line: line 3

//! Run-time feature detection on PowerPC64.

features! {
    @TARGET: powerpc64;
    @CFG: target_arch = "powerpc64";
    @MACRO_NAME: is_powerpc64_feature_detected;
