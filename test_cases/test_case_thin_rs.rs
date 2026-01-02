// MINIMAL TEST CASE for parsing failure in: ../rust/library/alloc/src/boxed/thin.rs
// Error: expected square brackets
// Problematic line: line 7


use core::error::Error;
use core::fmt::{self, Debug, Display, Formatter};
#[cfg(not(no_global_oom_handling))]
use core::intrinsics::{const_allocate, const_make_global};
use core::marker::PhantomData;
#[cfg(not(no_global_oom_handling))]
