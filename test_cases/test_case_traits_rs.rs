// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/bstr/traits.rs
// Error: expected square brackets
// Problematic line: line 8

use crate::slice::SliceIndex;
use crate::{hash, ops, range};

#[unstable(feature = "bstr", issue = "134915")]
impl Ord for ByteStr {
    #[inline]
    fn cmp(&self, other: &ByteStr) -> Ordering {
