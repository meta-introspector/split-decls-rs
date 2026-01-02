// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_mir_dataflow/src/framework/fmt.rs
// Error: expected square brackets
// Problematic line: line 11


use super::lattice::MaybeReachable;

/// An extension to `fmt::Debug` for data that can be better printed with some auxiliary data `C`.
pub trait DebugWithContext<C>: Eq + fmt::Debug {
    fn fmt_with(&self, _ctxt: &C, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
