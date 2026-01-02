// MINIMAL TEST CASE for parsing failure in: ../rust/library/std_detect/src/detect/arch/riscv.rs
// Error: expected square brackets
// Problematic line: line 3

//! Run-time feature detection on RISC-V.

features! {
    @TARGET: riscv;
    @CFG: any(target_arch = "riscv32", target_arch = "riscv64");
    @MACRO_NAME: is_riscv_feature_detected;
