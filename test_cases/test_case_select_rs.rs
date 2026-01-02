// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/slice/sort/select.rs
// Error: expected square brackets
// Problematic line: line 11


use crate::cfg_select;
use crate::mem::{self, SizedTypeProperties};
#[cfg(not(feature = "optimize_for_size"))]
use crate::slice::sort::shared::pivot::choose_pivot;
use crate::slice::sort::shared::smallsort::insertion_sort_shift_left;
use crate::slice::sort::unstable::quicksort::partition;
