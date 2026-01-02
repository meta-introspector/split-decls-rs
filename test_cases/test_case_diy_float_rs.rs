// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/num/diy_float.rs
// Error: expected square brackets
// Problematic line: line 6

// This module is only for dec2flt and flt2dec, and only public because of coretests.
// It is not intended to ever be stabilized.
#![doc(hidden)]
#![unstable(
    feature = "core_private_diy_float",
    reason = "internal routines only exposed for testing",
    issue = "none"
