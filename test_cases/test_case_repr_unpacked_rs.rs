// MINIMAL TEST CASE for parsing failure in: ../rust/library/std/src/io/error/repr_unpacked.rs
// Error: expected square brackets
// Problematic line: line 11


pub(super) struct Repr(Inner);

impl Repr {
    #[inline]
    pub(super) fn new(dat: ErrorData<Box<Custom>>) -> Self {
        Self(dat)
