// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_type_ir/src/walk.rs
// Error: expected square brackets
// Problematic line: line 15

// avoid heap allocations.
type TypeWalkerStack<I> = SmallVec<[<I as Interner>::GenericArg; 8]>;

/// An iterator for walking the type tree.
///
/// It's very easy to produce a deeply
/// nested type tree with a lot of
