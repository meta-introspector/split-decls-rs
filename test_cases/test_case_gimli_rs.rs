// MINIMAL TEST CASE for parsing failure in: ../rust/library/backtrace/src/symbolize/gimli.rs
// Error: expected square brackets
// Problematic line: line 21

use mystd::path::Path;
use mystd::prelude::v1::*;

#[cfg(backtrace_in_libstd)]
mod mystd {
    pub use crate::*;
}
