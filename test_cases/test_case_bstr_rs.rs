// MINIMAL TEST CASE for parsing failure in: ../rust/library/alloc/src/bstr.rs
// Error: expected square brackets
// Problematic line: line 7

#![cfg(not(no_global_oom_handling))]

use core::borrow::{Borrow, BorrowMut};
#[unstable(feature = "bstr", issue = "134915")]
pub use core::bstr::ByteStr;
use core::bstr::{impl_partial_eq, impl_partial_eq_n, impl_partial_eq_ord};
use core::cmp::Ordering;
