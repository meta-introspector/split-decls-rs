// MINIMAL TEST CASE for parsing failure in: ../rust/compiler/rustc_ast/src/expand/typetree.rs
// Error: expected square brackets
// Problematic line: line 26


use crate::expand::{Decodable, Encodable, HashStable_Generic};

#[derive(Clone, Copy, Eq, PartialEq, Encodable, Decodable, Debug, HashStable_Generic)]
pub enum Kind {
    Anything,
    Integer,
