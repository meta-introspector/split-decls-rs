// MINIMAL TEST CASE for parsing failure in: ../rust/library/stdarch/crates/core_arch/src/x86_64/rdrand.rs
// Error: expected square brackets
// Problematic line: line 7


#![allow(clippy::module_name_repetitions)]

#[allow(improper_ctypes)]
unsafe extern "unadjusted" {
    #[link_name = "llvm.x86.rdrand.64"]
    fn x86_rdrand64_step() -> (u64, i32);
