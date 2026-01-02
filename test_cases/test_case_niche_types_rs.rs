// MINIMAL TEST CASE for parsing failure in: ../rust/library/core/src/num/niche_types.rs
// Error: expected square brackets
// Problematic line: line 1

#![unstable(
    feature = "temporary_niche_types",
    issue = "none",
    reason = "for core, alloc, and std internals until pattern types are further along"
