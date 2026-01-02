// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/io/error/repr_bitpacked.rs
// Error: expected square brackets
// Problematic line: line 118

const TAG_OS: usize = 0b10;
const TAG_SIMPLE: usize = 0b11;

/// The internal representation.
///
/// See the module docs for more, this is just a way to hack in a check that we
/// indeed are not unwind-safe.
