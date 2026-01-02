// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_hir_analysis/src/impl_wf_check.rs
// Error: expected square brackets
// Problematic line: line 26


mod min_specialization;

/// Checks that all the type/lifetime parameters on an impl also
/// appear in the trait ref or self type (or are constrained by a
/// where-clause). These rules are needed to ensure that, given a
/// trait ref like `<T as Trait<U>>`, we can derive the values of all
