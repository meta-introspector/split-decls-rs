// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/marker/variance.rs
// Error: expected square brackets
// Problematic line: line 9

use crate::fmt;
use crate::hash::{Hash, Hasher};

macro_rules! first_token {
    ($first:tt $($rest:tt)*) => {
        $first
    };
