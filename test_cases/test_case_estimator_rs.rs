// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/num/flt2dec/estimator.rs
// Error: expected square brackets
// Problematic line: line 3

//! The exponent estimator.

/// Finds `k_0` such that `10^(k_0-1) < mant * 2^exp <= 10^(k_0+1)`.
///
/// This is used to approximate `k = ceil(log_10 (mant * 2^exp))`;
/// the true `k` is either `k_0` or `k_0+1`.
