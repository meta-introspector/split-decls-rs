// MINIMAL TEST CASE for parsing failure in: ../rust/library/alloc/src/str.rs
// Error: expected square brackets
// Problematic line: line 13

use core::borrow::{Borrow, BorrowMut};
use core::iter::FusedIterator;
use core::mem::MaybeUninit;
#[stable(feature = "encode_utf16", since = "1.8.0")]
pub use core::str::EncodeUtf16;
#[stable(feature = "split_ascii_whitespace", since = "1.34.0")]
pub use core::str::SplitAsciiWhitespace;
