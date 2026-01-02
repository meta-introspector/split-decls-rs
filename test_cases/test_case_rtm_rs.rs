// MINIMAL TEST CASE for parsing failure in: ../rust/library/stdarch/crates/core_arch/src/x86/rtm.rs
// Error: expected square brackets
// Problematic line: line 16

//! [wikipedia_rtm]: https://en.wikipedia.org/wiki/Transactional_Synchronization_Extensions#Restricted_Transactional_Memory
//! [intel_consid]: https://software.intel.com/en-us/cpp-compiler-developer-guide-and-reference-intel-transactional-synchronization-extensions-intel-tsx-programming-considerations

#[cfg(test)]
use stdarch_test::assert_instr;

unsafe extern "C" {
