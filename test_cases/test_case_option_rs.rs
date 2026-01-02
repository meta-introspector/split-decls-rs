// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/option.rs
// Error: expected square brackets
// Problematic line: line 586

use crate::pin::Pin;
use crate::{cmp, convert, hint, mem, slice};

/// The `Option` type. See [the module level documentation](self) for more.
#[doc(search_unbox)]
#[derive(Copy, Debug, Hash)]
#[derive_const(Eq)]
