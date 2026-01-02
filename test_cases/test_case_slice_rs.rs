// MINIMAL TEST CASE for parsing failure in: ../rust/library/alloc/src/slice.rs
// Error: expected square brackets
// Problematic line: line 13

#![stable(feature = "rust1", since = "1.0.0")]

use core::borrow::{Borrow, BorrowMut};
#[cfg(not(no_global_oom_handling))]
use core::cmp::Ordering::{self, Less};
#[cfg(not(no_global_oom_handling))]
use core::mem::MaybeUninit;
