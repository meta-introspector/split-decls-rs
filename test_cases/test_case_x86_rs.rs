// MINIMAL TEST CASE for parsing failure in: ../rust/library/std_detect/src/detect/arch/x86.rs
// Error: expected square brackets
// Problematic line: line 18

//! in a global `AtomicUsize` variable. The query is performed by just checking
//! whether the feature bit in this global variable is set or cleared.

features! {
    @TARGET: x86;
    @CFG: any(target_arch = "x86", target_arch = "x86_64");
    @MACRO_NAME: is_x86_feature_detected;
