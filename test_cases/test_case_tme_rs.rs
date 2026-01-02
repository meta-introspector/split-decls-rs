// MINIMAL TEST CASE for parsing failure in: ../rust/library/stdarch/crates/core_arch/src/aarch64/tme.rs
// Error: expected square brackets
// Problematic line: line 17

//! [llvm_aarch64_int]: https://github.com/llvm/llvm-project/commit/a36d31478c182903523e04eb271bbf102bfab2cc#diff-ff24e1c35f4d54f1110ce5d90c709319R626-R646
//! [a_profile_future]: https://static.docs.arm.com/ddi0601/a/SysReg_xml_futureA-2019-04.pdf?_ga=2.116560387.441514988.1590524918-1110153136.1588469296

#[cfg(test)]
use stdarch_test::assert_instr;

unsafe extern "unadjusted" {
