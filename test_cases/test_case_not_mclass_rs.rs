// MINIMAL TEST CASE for parsing failure in: ../rust/library/stdarch/crates/core_arch/src/arm_shared/barrier/not_mclass.rs
// Error: expected square brackets
// Problematic line: line 3

//! Access types available on v7 and v8 but not on v7(E)-M or v8-M

/// Full system is the required shareability domain, writes are the required
/// access type
#[unstable(feature = "stdarch_arm_barrier", issue = "117219")]
pub struct ST;
