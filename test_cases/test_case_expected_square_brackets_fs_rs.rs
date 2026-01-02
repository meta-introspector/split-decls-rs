// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/os/emscripten/fs.rs
// Error: expected square brackets
// Error type: expected_square_brackets
// Sample #3 of 3
// Problematic line: line 4

#![stable(feature = "metadata_ext", since = "1.1.0")]

use crate::fs::Metadata;
#[allow(deprecated)]
use crate::os::emscripten::raw;
use crate::sys_common::AsInner;

