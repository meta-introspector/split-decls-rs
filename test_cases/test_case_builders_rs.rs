// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/fmt/builders.rs
// Error: expected square brackets
// Problematic line: line 5


use crate::fmt::{self, Debug, Formatter};

struct PadAdapter<'buf, 'state> {
    buf: &'buf mut (dyn fmt::Write + 'buf),
    state: &'state mut PadAdapterState,
}
