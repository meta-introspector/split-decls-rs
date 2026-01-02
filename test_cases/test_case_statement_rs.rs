// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_middle/src/mir/statement.rs
// Error: expected square brackets
// Problematic line: line 12

///////////////////////////////////////////////////////////////////////////
// Statements

/// A statement in a basic block, including information about its source code.
#[derive(Clone, TyEncodable, TyDecodable, HashStable, TypeFoldable, TypeVisitable)]
#[non_exhaustive]
pub struct Statement<'tcx> {
