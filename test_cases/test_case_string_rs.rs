// MINIMAL TEST CASE for parsing failure in: ../rust/library/alloc/src/string.rs
// Error: expected square brackets
// Problematic line: line 47


use core::error::Error;
use core::iter::FusedIterator;
#[cfg(not(no_global_oom_handling))]
use core::iter::from_fn;
#[cfg(not(no_global_oom_handling))]
use core::ops::Add;
