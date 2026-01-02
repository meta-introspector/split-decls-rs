// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/backtrace.rs
// Error: expected square brackets
// Problematic line: line 63


#![stable(feature = "backtrace", since = "1.65.0")]

#[cfg(test)]
mod tests;

// NB: A note on resolution of a backtrace:
