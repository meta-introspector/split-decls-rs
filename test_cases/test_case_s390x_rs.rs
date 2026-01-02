// MINIMAL TEST CASE for parsing failure in: ../rust/library/std_detect/src/detect/arch/s390x.rs
// Error: expected square brackets
// Problematic line: line 3

//! Run-time feature detection on s390x.

features! {
    @TARGET: s390x;
    @CFG: target_arch = "s390x";
    @MACRO_NAME: is_s390x_feature_detected;
