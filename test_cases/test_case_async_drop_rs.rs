// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/future/async_drop.rs
// Error: expected square brackets
// Problematic line: line 3

#![unstable(feature = "async_drop", issue = "126482")]

#[allow(unused_imports)]
use core::future::Future;

#[allow(unused_imports)]
