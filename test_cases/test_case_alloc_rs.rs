// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/sys/pal/sgx/abi/usercalls/alloc.rs
// Error: expected square brackets
// Problematic line: line 16

use crate::slice::SliceIndex;
use crate::{cmp, intrinsics, slice};

/// A type that can be safely read from or written to userspace.
///
/// Non-exhaustive list of specific requirements for reading and writing:
/// * **Type is `Copy`** (and therefore also not `Drop`). Copies will be
