// MINIMAL TEST CASE for parsing failure in: ../rust/library/alloc/src/boxed.rs
// Error: expected square brackets
// Problematic line: line 187

#![stable(feature = "rust1", since = "1.0.0")]

use core::borrow::{Borrow, BorrowMut};
#[cfg(not(no_global_oom_handling))]
use core::clone::CloneToUninit;
use core::cmp::Ordering;
use core::error::{self, Error};
