// MINIMAL TEST CASE for parsing failure in: ../rust/library/stdarch/crates/core_arch/src/arm_shared/barrier/common.rs
// Error: expected square brackets
// Problematic line: line 3

//! Access types available on all architectures

/// Full system is the required shareability domain, reads and writes are the
/// required access types
#[unstable(feature = "stdarch_arm_barrier", issue = "117219")]
pub struct SY;
