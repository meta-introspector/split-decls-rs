// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_data_structures/src/thinvec.rs
// Error: expected square brackets
// Problematic line: line 9


use thin_vec::ThinVec;

/// An iterator for [`ThinVec`] which uses a closure to determine if an element should be removed.
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct ExtractIf<'a, T, F> {
    vec: &'a mut ThinVec<T>,
