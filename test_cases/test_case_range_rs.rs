// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/range.rs
// Error: expected square brackets
// Problematic line: line 24


mod iter;

#[unstable(feature = "new_range_api", issue = "125687")]
pub mod legacy;

use Bound::{Excluded, Included, Unbounded};
