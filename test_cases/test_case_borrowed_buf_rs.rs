// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/io/borrowed_buf.rs
// Error: expected square brackets
// Problematic line: line 7

use crate::mem::{self, MaybeUninit};
use crate::{cmp, ptr};

/// A borrowed byte buffer which is incrementally filled and initialized.
///
/// This type is a sort of "double cursor". It tracks three regions in the buffer: a region at the beginning of the
/// buffer that has been logically filled with data, a region that has been initialized at some point but not yet
