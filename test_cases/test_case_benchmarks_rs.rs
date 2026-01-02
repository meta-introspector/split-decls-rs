// MINIMAL TEST CASE for parsing failure in: ../rust/library/backtrace/benches/benchmarks.rs
// Error: expected square brackets
// Problematic line: line 5


extern crate test;

#[cfg(feature = "std")]
use backtrace::Backtrace;

#[bench]
